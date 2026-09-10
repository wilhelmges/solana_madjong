@echo off
setlocal

echo [1/2] Building release...
cargo build --release
if errorlevel 1 (echo BUILD FAILED & exit /b 1)

echo [2/2] Copying to project root...
copy target\release\solong.exe solong.exe >nul

echo.
echo BUILD SUCCESSFUL: solong.exe
