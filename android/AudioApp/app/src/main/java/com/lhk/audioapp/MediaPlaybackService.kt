package com.lhk.audioapp

import android.drm.DrmStore
import android.media.session.MediaSession
import android.os.Bundle
import android.support.v4.media.MediaBrowserCompat
import android.support.v4.media.session.MediaSessionCompat
import android.support.v4.media.session.PlaybackStateCompat
import androidx.core.app.NotificationCompat
import androidx.core.content.ContextCompat
import androidx.media.MediaBrowserServiceCompat
import androidx.media.session.MediaButtonReceiver

private const val MY_MEDIA_ROOT_ID = "media_root_id"
private const val MY_EMPTY_MEDIA_ROOT_ID = "empty_root_id"
private const val LOG_TAG = "MeidaService"

class MediaPlaybackService : MediaBrowserServiceCompat() {
    private lateinit var mediaSession: MediaSessionCompat
    private lateinit var stateBuilder: PlaybackStateCompat.Builder

    override fun onCreate() {
        super.onCreate()

        mediaSession = MediaSessionCompat(baseContext, LOG_TAG).apply {
            setFlags(
                MediaSessionCompat.FLAG_HANDLES_MEDIA_BUTTONS
                        or MediaSessionCompat.FLAG_HANDLES_TRANSPORT_CONTROLS
            )

            stateBuilder = PlaybackStateCompat.Builder()
                .setActions(
                    PlaybackStateCompat.ACTION_PLAY
                            or PlaybackStateCompat.ACTION_PLAY_PAUSE
                )

            setPlaybackState(stateBuilder.build())

            setCallback(MysessionCallback)

            setSessionToken(sessionToken)
        }
        sessionToken = mediaSession.sessionToken
    }

    override fun onLoadChildren(
        parentMediaId: String,
        result: Result<List<MediaBrowserCompat.MediaItem>>
    ) {
        if (MY_EMPTY_MEDIA_ROOT_ID == parentMediaId) {
            result.sendResult(null)
            return
        }

        val mediaItems = emptyList<MediaBrowserCompat.MediaItem>()

        if (MY_MEDIA_ROOT_ID == parentMediaId) {

        } else {

        }
        result.sendResult(mediaItems)
    }

    override fun onGetRoot(
        clientPackageName: String,
        clientUid: Int,
        rootHints: Bundle?
    ): BrowserRoot? {
        return if (allowBrowsing(clientPackageName, clientUid)) {
            BrowserRoot(MY_MEDIA_ROOT_ID, null)
        } else {
            BrowserRoot(MY_EMPTY_MEDIA_ROOT_ID, null)
        }
    }

    fun allowBrowsing(clientPackageName: String, clientUid: Int): Boolean {
        return true;
    }

    val MysessionCallback = object : MediaSessionCompat.Callback() {
        override fun onPlay() {
            super.onPlay()
        }

        override fun onPause() {
            super.onPause()
        }
    }
}