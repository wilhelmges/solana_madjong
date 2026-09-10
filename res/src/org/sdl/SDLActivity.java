package com.solong.mahjong;

import android.app.Activity;
import android.os.Bundle;

public class SDLActivity extends Activity {
    static {
        System.loadLibrary("solong");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
    }
}
