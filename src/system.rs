use anyhow::{anyhow, Result};
use core::ptr;
use crate::fmod_call;

pub struct System(*mut sys::FMOD_STUDIO_SYSTEM);

type AdvancedSettings = sys::FMOD_STUDIO_ADVANCEDSETTINGS;

impl System {
    pub fn new() -> Result<Self> {
        let mut system: *mut sys::FMOD_STUDIO_SYSTEM = ptr::null_mut();
        fmod_call!(FMOD_Studio_System_Create, &mut system, sys::FMOD_VERSION => System(system))
    }

    pub fn init(&mut self) -> Result<()> {
        fmod_call!(FMOD_Studio_System_Initialize, self.0, 512, sys::FMOD_STUDIO_INIT_NORMAL,
                   sys::FMOD_INIT_NORMAL, ptr::null_mut() => ())
    }

    pub fn update(&mut self) -> Result<()> {
        fmod_call!(FMOD_Studio_System_Update, self.0 => ())
    }

    pub fn is_valid(&self) -> bool {
        let result = unsafe { sys::FMOD_Studio_System_IsValid(self.0) };
        if result == 1 {
            true
        } else {
            false
        }
    }

    pub fn advanced_settings(&self) -> Result<AdvancedSettings> {
        let setting: *mut AdvancedSettings = ptr::null_mut();
        fmod_call!(FMOD_Studio_System_GetAdvancedSettings, self.0, setting => unsafe {*setting})
    }

    pub fn set_advanced_settings(&mut self, settings: &mut AdvancedSettings) -> Result<()> {
        fmod_call!(FMOD_Studio_System_SetAdvancedSettings, self.0, settings => ())
    }

    pub fn flush_commands(&mut self) -> Result<()> {
        fmod_call!(FMOD_Studio_System_FlushCommands, self.0 => ())
    }

    pub fn flush_sample_loading(&mut self) -> Result<()> {
        fmod_call!(FMOD_Studio_System_FlushSampleLoading, self.0 => ())
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe { sys::FMOD_Studio_System_Release(self.0); }
    }
}
