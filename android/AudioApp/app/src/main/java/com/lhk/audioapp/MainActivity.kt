package com.lhk.audioapp

import android.content.ComponentName
import android.os.Bundle
import android.support.v4.media.MediaBrowserCompat
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    private lateinit var mediaBrowser: MediaBrowserCompat;

    override fun onCreate(savedInstanceState: Bundle?){
        super.onCreate(savedInstanceState)

        mediaBrowser = MediaBrowserCompat(this,
            ComponentName(this, MediaPlaybackService::class.java)
        )
    }
}