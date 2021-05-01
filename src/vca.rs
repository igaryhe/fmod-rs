use crate::{fmod_call, GUID};
use anyhow::{anyhow, Result};
use core::ptr;
use std::ffi::CString;

pub struct Vca(*mut sys::FMOD_STUDIO_VCA);

impl Vca {
    pub unsafe fn new(vca: *mut sys::FMOD_STUDIO_VCA) -> Self {
        Self(vca)
    }

    pub fn is_valid(&self) -> bool {
        let result = unsafe { sys::FMOD_Studio_VCA_IsValid(self.0) };
        if result == 1 {
            true
        } else {
            false
        }
    }

    pub fn id(&self) -> Result<GUID> {
        let id: *mut GUID = ptr::null_mut();
        fmod_call!(FMOD_Studio_VCA_GetID, self.0, id => unsafe {*id})
    }

    pub fn path(&self) -> Result<String> {
        let mut len = 0;
        unsafe { sys::FMOD_Studio_VCA_GetPath(self.0, ptr::null_mut(), 0, &mut len) };
        let path = vec![0; len as usize];
        let path_ptr = CString::new(path)?.into_raw();
        fmod_call!(FMOD_Studio_VCA_GetPath, self.0, path_ptr, len, &mut len =>
                   unsafe { CString::from_raw(path_ptr).into_string()?})
    }

    pub fn volume(&self) -> Result<(f32, f32)> {
        let mut volume = 0.0;
        let mut final_volume = 0.0;
        fmod_call!(FMOD_Studio_VCA_GetVolume, self.0, &mut volume, &mut final_volume => (volume, final_volume))
    }

    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        fmod_call!(FMOD_Studio_VCA_SetVolume, self.0, volume => ())
    }
}
