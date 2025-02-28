use crate::session_description::{self, SdpParseError, SdpType};
use cxx::UniquePtr;
use webrtc_sys::jsep as sys_jsep;

impl From<sys_jsep::ffi::SdpType> for SdpType {
    fn from(sdp_type: sys_jsep::ffi::SdpType) -> Self {
        match sdp_type {
            sys_jsep::ffi::SdpType::Offer => SdpType::Offer,
            sys_jsep::ffi::SdpType::PrAnswer => SdpType::PrAnswer,
            sys_jsep::ffi::SdpType::Answer => SdpType::Answer,
            sys_jsep::ffi::SdpType::Rollback => SdpType::Rollback,
            _ => panic!("unknown SdpType"),
        }
    }
}

impl From<SdpType> for sys_jsep::ffi::SdpType {
    fn from(sdp_type: SdpType) -> Self {
        match sdp_type {
            SdpType::Offer => sys_jsep::ffi::SdpType::Offer,
            SdpType::PrAnswer => sys_jsep::ffi::SdpType::PrAnswer,
            SdpType::Answer => sys_jsep::ffi::SdpType::Answer,
            SdpType::Rollback => sys_jsep::ffi::SdpType::Rollback,
        }
    }
}

impl From<sys_jsep::ffi::SdpParseError> for SdpParseError {
    fn from(e: sys_jsep::ffi::SdpParseError) -> Self {
        Self {
            line: e.line,
            description: e.description,
        }
    }
}

pub struct SessionDescription {
    pub(crate) sys_handle: UniquePtr<sys_jsep::ffi::SessionDescription>,
}

impl SessionDescription {
    pub fn parse(
        sdp: &str,
        sdp_type: SdpType,
    ) -> Result<session_description::SessionDescription, SdpParseError> {
        let res = sys_jsep::ffi::create_session_description(sdp_type.into(), sdp.to_owned());
        match res {
            Ok(sys_handle) => Ok(session_description::SessionDescription {
                handle: SessionDescription { sys_handle },
            }),
            Err(e) => Err(unsafe { sys_jsep::ffi::SdpParseError::from(e.what()).into() }),
        }
    }

    pub fn sdp_type(&self) -> SdpType {
        self.sys_handle.sdp_type().into()
    }
}

impl ToString for SessionDescription {
    fn to_string(&self) -> String {
        self.sys_handle.stringify()
    }
}

impl Clone for SessionDescription {
    fn clone(&self) -> Self {
        SessionDescription {
            sys_handle: self.sys_handle.clone(),
        }
    }
}
