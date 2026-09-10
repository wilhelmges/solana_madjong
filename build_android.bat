@echo off
setlocal enabledelayedexpansion

call setenv.bat

set APK_NAME=Solong
set PACKAGE=com.solong.mahjong
set MIN_SDK=28
set TARGET_SDK=35

set ANDROID_JAR=%LOCALAPPDATA%\Android\Sdk\platforms\android-35\android.jar
set AAPT2=%LOCALAPPDATA%\Android\Sdk\build-tools\35.0.0\aapt2.exe
set AAPT=%LOCALAPPDATA%\Android\Sdk\build-tools\35.0.0\aapt.exe
set D8=%LOCALAPPDATA%\Android\Sdk\build-tools\35.0.0\d8.exe
set ZIPALIGN=%LOCALAPPDATA%\Android\Sdk\build-tools\35.0.0\zipalign.exe
set APKSIGNER=%LOCALAPPDATA%\Android\Sdk\build-tools\35.0.0\apksigner.bat
set KEYSTORE=debug.keystore

set BUILD_DIR=target\android
set GEN_DIR=%BUILD_DIR%\gen
set OBJ_DIR=%BUILD_DIR%\obj

echo [1/6] Building Rust library for Android...
cargo ndk -t arm64-v8a build --release
if errorlevel 1 (echo BUILD FAILED & exit /b 1)

echo [2/6] Compiling Java source...
if exist "%OBJ_DIR%" rmdir /s /q "%OBJ_DIR%"
mkdir "%OBJ_DIR%"
javac --release 8 -classpath "%ANDROID_JAR%" -d "%OBJ_DIR%" res\src\com\solong\mahjong\MainActivity.java res\src\quad_native\QuadNative.java
if errorlevel 1 (echo JAVA COMPILE FAILED & exit /b 1)

echo [3/6] Creating classes.dex...
dir /s /b "%OBJ_DIR%\*.class" > "%BUILD_DIR%\classes_list.txt"
%D8% --lib "%ANDROID_JAR%" --output "%BUILD_DIR%" @%BUILD_DIR%\classes_list.txt
if errorlevel 1 (echo D8 FAILED & exit /b 1)

echo [4/6] Linking APK...
if not exist "%GEN_DIR%" mkdir "%GEN_DIR%"
%AAPT2% link -o "%BUILD_DIR%\%APK_NAME%-unsigned.apk" ^
    -I "%ANDROID_JAR%" ^
    --manifest res/AndroidManifest.xml ^
    --auto-add-overlay ^
    --min-sdk-version %MIN_SDK% ^
    --target-sdk-version %TARGET_SDK%
if errorlevel 1 (echo AAPT2 LINK FAILED & exit /b 1)

echo [5/6] Adding files to APK...
set PROJECT_ROOT=%~dp0
mkdir "%BUILD_DIR%\apk_contents\lib\arm64-v8a" 2>nul
copy target\aarch64-linux-android\release\libsolong.so "%BUILD_DIR%\apk_contents\lib\arm64-v8a\" >nul
copy "%BUILD_DIR%\classes.dex" "%BUILD_DIR%\apk_contents\" >nul
pushd "%BUILD_DIR%\apk_contents"
"%AAPT%" add "%PROJECT_ROOT%%BUILD_DIR%\%APK_NAME%-unsigned.apk" classes.dex lib/arm64-v8a/libsolong.so
popd
if errorlevel 1 (echo ADD FILES FAILED & exit /b 1)

echo [6/6] Aligning and signing APK...
%ZIPALIGN% -f 4 "%BUILD_DIR%\%APK_NAME%-unsigned.apk" "%BUILD_DIR%\%APK_NAME%-aligned.apk"
if errorlevel 1 (echo ZIPALIGN FAILED & exit /b 1)

if not exist "%KEYSTORE%" (
    keytool -genkeypair -v -keystore "%KEYSTORE%" -alias debug -keyalg RSA -keysize 2048 -validity 10000 ^
        -storepass android -keypass android ^
        -dname "CN=Debug,O=Debug,C=US"
)
%APKSIGNER% sign --ks "%KEYSTORE%" --ks-pass pass:android --key-pass pass:android ^
    --out "%BUILD_DIR%\%APK_NAME%.apk" "%BUILD_DIR%\%APK_NAME%-aligned.apk"
if errorlevel 1 (echo SIGN FAILED & exit /b 1)

echo.
echo BUILD SUCCESSFUL: %BUILD_DIR%\%APK_NAME%.apk