use crate::imp::rtp_receiver::RtpReceiver;
use crate::imp::rtp_sender::RtpSender;
use crate::rtp_parameters::RtpCodecCapability;
use crate::rtp_receiver;
use crate::rtp_sender;
use crate::rtp_transceiver::RtpTransceiverDirection;
use crate::rtp_transceiver::RtpTransceiverInit;
use crate::RtcError;
use cxx::SharedPtr;
use webrtc_sys::rtc_error as sys_err;
use webrtc_sys::rtp_transceiver as sys_rt;
use webrtc_sys::webrtc as sys_webrtc;

impl From<sys_webrtc::ffi::RtpTransceiverDirection> for RtpTransceiverDirection {
    fn from(value: sys_webrtc::ffi::RtpTransceiverDirection) -> Self {
        match value {
            sys_webrtc::ffi::RtpTransceiverDirection::SendRecv => Self::SendRecv,
            sys_webrtc::ffi::RtpTransceiverDirection::SendOnly => Self::SendOnly,
            sys_webrtc::ffi::RtpTransceiverDirection::RecvOnly => Self::RecvOnly,
            sys_webrtc::ffi::RtpTransceiverDirection::Inactive => Self::Inactive,
            sys_webrtc::ffi::RtpTransceiverDirection::Stopped => Self::Stopped,
            _ => panic!("unknown RtpTransceiverDirection"),
        }
    }
}

impl From<RtpTransceiverDirection> for sys_webrtc::ffi::RtpTransceiverDirection {
    fn from(value: RtpTransceiverDirection) -> Self {
        match value {
            RtpTransceiverDirection::SendRecv => Self::SendRecv,
            RtpTransceiverDirection::SendOnly => Self::SendOnly,
            RtpTransceiverDirection::RecvOnly => Self::RecvOnly,
            RtpTransceiverDirection::Inactive => Self::Inactive,
            RtpTransceiverDirection::Stopped => Self::Stopped,
        }
    }
}

impl From<RtpTransceiverInit> for sys_rt::ffi::RtpTransceiverInit {
    fn from(value: RtpTransceiverInit) -> Self {
        Self {
            direction: value.direction.into(),
            stream_ids: value.stream_ids,
            send_encodings: value.send_encodings.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone)]
pub struct RtpTransceiver {
    pub(crate) sys_handle: SharedPtr<sys_rt::ffi::RtpTransceiver>,
}

impl RtpTransceiver {
    pub fn mid(&self) -> Option<String> {
        self.sys_handle.mid().ok()
    }

    pub fn current_direction(&self) -> Option<RtpTransceiverDirection> {
        self.sys_handle.current_direction().ok().map(Into::into)
    }

    pub fn direction(&self) -> RtpTransceiverDirection {
        self.sys_handle.direction().into()
    }

    pub fn sender(&self) -> rtp_sender::RtpSender {
        rtp_sender::RtpSender {
            handle: RtpSender {
                sys_handle: self.sys_handle.sender(),
            },
        }
    }

    pub fn receiver(&self) -> rtp_receiver::RtpReceiver {
        rtp_receiver::RtpReceiver {
            handle: RtpReceiver {
                sys_handle: self.sys_handle.receiver(),
            },
        }
    }

    pub fn set_codec_preferences(&self, codecs: Vec<RtpCodecCapability>) -> Result<(), RtcError> {
        self.sys_handle
            .set_codec_preferences(codecs.into_iter().map(Into::into).collect())
            .map_err(|e| unsafe { sys_err::ffi::RTCError::from(e.what()).into() })
    }

    pub fn stop(&self) -> Result<(), RtcError> {
        self.sys_handle
            .stop_standard()
            .map_err(|e| unsafe { sys_err::ffi::RTCError::from(e.what()).into() })
    }
}
