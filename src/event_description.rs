use anyhow::{anyhow, Result};
use core::ptr;
use crate::{common::GUID, bus::Bus, vca::Vca};
use std::ffi::CString;

pub struct EventDescription(*mut sys::FMOD_STUDIO_EVENTDESCRIPTION);

impl EventDescription {
    pub unsafe fn new(event_desc: *mut sys::FMOD_STUDIO_EVENTDESCRIPTION) -> Self {
        Self(event_desc)
    }
}
