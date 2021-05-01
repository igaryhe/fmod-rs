use anyhow::{anyhow, Result};
use core::ptr;
use sys::{FMOD_RESULT, FMOD_STUDIO_ADVANCEDSETTINGS, FMOD_STUDIO_SYSTEM};

pub struct System(*mut FMOD_STUDIO_SYSTEM);

type AdvancedSettings = FMOD_STUDIO_ADVANCEDSETTINGS;

impl System {
    pub fn new() -> Result<Self> {
        use sys::{FMOD_Studio_System_Create, FMOD_VERSION};
        unsafe {
            let mut system: *mut FMOD_STUDIO_SYSTEM = ptr::null_mut();
            let result = FMOD_Studio_System_Create(&mut system, FMOD_VERSION);
            if result == FMOD_RESULT::FMOD_OK {
                Ok(System(system))
            } else {
                Err(anyhow!(result))
            }
        }
    }

    pub fn init(&mut self) -> Result<()> {
        use sys::{FMOD_Studio_System_Initialize, FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL};
        unsafe {
            let result = FMOD_Studio_System_Initialize(
                self.0,
                512,
                FMOD_STUDIO_INIT_NORMAL,
                FMOD_INIT_NORMAL,
                ptr::null_mut(),
            );
            if result == FMOD_RESULT::FMOD_OK {
                Ok(())
            } else {
                Err(anyhow!(result))
            }
        }
    }

    pub fn update(&mut self) -> Result<()> {
        use sys::FMOD_Studio_System_Update;
        unsafe {
            let result = FMOD_Studio_System_Update(self.0);
            if result == FMOD_RESULT::FMOD_OK {
                Ok(())
            } else {
                Err(anyhow!(result))
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        use sys::FMOD_Studio_System_IsValid;
        unsafe {
            let result = FMOD_Studio_System_IsValid(self.0);
            if result == 1 {
                true
            } else {
                false
            }
        }
    }

    pub fn advanced_settings(&self) -> Result<AdvancedSettings> {
        use sys::FMOD_Studio_System_GetAdvancedSettings;
        unsafe {
            let setting: *mut AdvancedSettings = ptr::null_mut();
            let result = FMOD_Studio_System_GetAdvancedSettings(self.0, setting);
            if result == FMOD_RESULT::FMOD_OK {
                Ok(*setting)
            } else {
                Err(anyhow!(result))
            }
        }
    }

    pub fn set_advanced_settings(&mut self, settings: &mut AdvancedSettings) -> Result<()> {
        use sys::FMOD_Studio_System_SetAdvancedSettings;
        unsafe {
            let result = FMOD_Studio_System_SetAdvancedSettings(self.0, settings);
            if result == FMOD_RESULT::FMOD_OK {
                Ok(())
            } else {
                Err(anyhow!(result))
            }
        }
    }

    pub fn flush_commands(&mut self) -> Result<()> {
        use sys::FMOD_Studio_System_FlushCommands;
        unsafe {
            let result = FMOD_Studio_System_FlushCommands(self.0);
            if result == FMOD_RESULT::FMOD_OK {
                Ok(())
            } else {
                Err(anyhow!(result))
            }
        }
    }

    pub fn flush_sample_loading(&mut self) -> Result<()> {
        use sys::FMOD_Studio_System_FlushSampleLoading;
        unsafe {
            let result = FMOD_Studio_System_FlushSampleLoading(self.0);
            if result == FMOD_RESULT::FMOD_OK {
                Ok(())
            } else {
                Err(anyhow!(result))
            }
        }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        use sys::FMOD_Studio_System_Release;
        unsafe {
            FMOD_Studio_System_Release(self.0);
        }
    }
}
