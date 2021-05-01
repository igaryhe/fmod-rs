use std::ffi::CString;
use core::ptr;
use anyhow::{anyhow, Result};
use crate::fmod_call;

bitflags! {
    pub struct Initflags: u32 {
        const NORMAL                = 0x00000000;
        const LIVEUPDATE            = 0x00000001;
        const ALLOW_MISSING_PLUGINS = 0x00000002;
        const SYNCHRONOUS_UPDATE    = 0x00000004;
        const DEFERRED_CALLBACKS    = 0x00000008;
        const LOAD_FROM_UPDATE      = 0x00000010;
        const MEMORY_TRACING        = 0x00000020;
    }
}

bitflags! {
    pub struct ParameterFlags: u32 {
        const READONLY  = 0x00000001;
        const AUTOMATIC = 0x00000002;
        const GLOBAL    = 0x00000004;
        const DISCRETE  = 0x00000008;
    }
}

bitflags! {
    struct SystemCallbackType: u32 {
        const PREUPDATE               = 0x00000001;
        const POSTUPDATE              = 0x00000002;
        const BANK_UNLOAD             = 0x00000004;
        const LIVEUPDATE_CONNECTED    = 0x00000008;
        const LIVEUPDATE_DISCONNECTED = 0x00000010;
        const ALL                     = 0xffffffff;
    }
}

bitflags! {
    struct EventCallbackType: u32 {
        const CREATED                  = 0x00000001;
        const DESTROYED                = 0x00000002;
        const STARTING                 = 0x00000004;
        const STARTED                  = 0x00000008;
        const RESTARTED                = 0x00000010;
        const STOPPED                  = 0x00000020;
        const START_FAILED             = 0x00000040;
        const CREATE_PROGRAMMER_SOUND  = 0x00000080;
        const DESTROY_PROGRAMMER_SOUND = 0x00000100;
        const PLUGIN_CREATED           = 0x00000200;
        const PLUGIN_DESTROYED         = 0x00000400;
        const TIMELINE_MARKER          = 0x00000800;
        const TIMELINE_BEAT            = 0x00001000;
        const SOUND_PLAYED             = 0x00002000;
        const SOUND_STOPPED            = 0x00004000;
        const REAL_TO_VIRTUAL          = 0x00008000;
        const VIRTUAL_TO_REAL          = 0x00010000;
        const START_EVENT_COMMAND      = 0x00020000;
        const ALL                      = 0xffffffff;
    }
}

bitflags! {
    struct LoadBankFlags: u32 {
        const NORMAL             = 0x00000000;
        const NONBLOCKING        = 0x00000001;
        const DECOMPRESS_SAMPLES = 0x00000002;
        const UNENCRYPTED        = 0x00000004;
    }
}

bitflags! {
    struct CommandcaptureFlags: u32 {
        const NORMAL             = 0x00000000;
        const FILEFLUSH          = 0x00000001;
        const SKIP_INITIAL_STATE = 0x00000002;
    }
}

bitflags! {
    struct CommandreplayFlags: u32 {
        const NORMAL         = 0x00000000;
        const SKIP_CLEANUP   = 0x00000001;
        const FAST_FORWARD   = 0x00000002;
        const SKIP_BAND_LOAD = 0x00000004;
    }
}

pub type GUID = sys::FMOD_GUID;

pub fn parse_id(id: &str) -> Result<GUID> {
    let raw_id = CString::new(id).expect("CString::new failed");
    let out: *mut GUID = ptr::null_mut();
    fmod_call!(FMOD_Studio_ParseID, raw_id.as_ptr(), out => unsafe {*out})
}
