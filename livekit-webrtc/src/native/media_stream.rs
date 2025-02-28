use crate::media_stream::{self, MediaStreamTrack, RtcTrackState};
use cxx::SharedPtr;
use webrtc_sys::media_stream as sys_ms;
use webrtc_sys::media_stream::ffi::{
    audio_to_media, media_to_audio, media_to_video, video_to_media,
};
use webrtc_sys::{MEDIA_TYPE_AUDIO, MEDIA_TYPE_VIDEO};

impl From<sys_ms::ffi::TrackState> for RtcTrackState {
    fn from(state: sys_ms::ffi::TrackState) -> Self {
        match state {
            sys_ms::ffi::TrackState::Live => RtcTrackState::Live,
            sys_ms::ffi::TrackState::Ended => RtcTrackState::Ended,
            _ => panic!("unknown TrackState"),
        }
    }
}

#[derive(Clone)]
pub struct MediaStream {
    pub(crate) sys_handle: SharedPtr<sys_ms::ffi::MediaStream>,
}

impl MediaStream {
    pub fn id(&self) -> String {
        self.sys_handle.id()
    }

    pub fn audio_tracks(&self) -> Vec<media_stream::RtcAudioTrack> {
        self.sys_handle
            .get_audio_tracks()
            .into_iter()
            .map(|t| media_stream::RtcAudioTrack {
                handle: RtcAudioTrack { sys_handle: t.ptr },
            })
            .collect()
    }

    pub fn video_tracks(&self) -> Vec<media_stream::RtcVideoTrack> {
        self.sys_handle
            .get_video_tracks()
            .into_iter()
            .map(|t| media_stream::RtcVideoTrack {
                handle: RtcVideoTrack { sys_handle: t.ptr },
            })
            .collect()
    }
}

pub fn new_media_stream_track(
    sys_handle: SharedPtr<sys_ms::ffi::MediaStreamTrack>,
) -> MediaStreamTrack {
    if sys_handle.kind() == MEDIA_TYPE_AUDIO {
        MediaStreamTrack::Audio(media_stream::RtcAudioTrack {
            handle: RtcAudioTrack {
                sys_handle: media_to_audio(sys_handle),
            },
        })
    } else if sys_handle.kind() == MEDIA_TYPE_VIDEO {
        MediaStreamTrack::Video(media_stream::RtcVideoTrack {
            handle: RtcVideoTrack {
                sys_handle: media_to_video(sys_handle),
            },
        })
    } else {
        panic!("unknown track kind")
    }
}

macro_rules! impl_media_stream_track {
    ($cast:ident) => {
        pub fn id(&self) -> String {
            let ptr = sys_ms::ffi::$cast(self.sys_handle.clone());
            ptr.id()
        }

        pub fn enabled(&self) -> bool {
            let ptr = sys_ms::ffi::$cast(self.sys_handle.clone());
            ptr.enabled()
        }

        pub fn set_enabled(&self, enabled: bool) -> bool {
            let ptr = sys_ms::ffi::$cast(self.sys_handle.clone());
            ptr.set_enabled(enabled)
        }

        pub fn state(&self) -> RtcTrackState {
            let ptr = sys_ms::ffi::$cast(self.sys_handle.clone());
            ptr.state().into()
        }
    };
}

#[derive(Clone)]
pub struct RtcVideoTrack {
    pub(crate) sys_handle: SharedPtr<sys_ms::ffi::VideoTrack>,
}

impl RtcVideoTrack {
    impl_media_stream_track!(video_to_media);

    pub fn sys_handle(&self) -> SharedPtr<sys_ms::ffi::MediaStreamTrack> {
        video_to_media(self.sys_handle.clone())
    }
}

#[derive(Clone)]
pub struct RtcAudioTrack {
    pub(crate) sys_handle: SharedPtr<sys_ms::ffi::AudioTrack>,
}

impl RtcAudioTrack {
    impl_media_stream_track!(audio_to_media);

    pub fn sys_handle(&self) -> SharedPtr<sys_ms::ffi::MediaStreamTrack> {
        audio_to_media(self.sys_handle.clone())
    }
}
