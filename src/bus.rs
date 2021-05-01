use crate::common::GUID;
use crate::fmod_call;
use anyhow::{anyhow, Result};
use core::ptr;
use std::ffi::CString;

pub struct Bus(*mut sys::FMOD_STUDIO_BUS);

pub struct Channelgroup(*mut sys::FMOD_CHANNELGROUP);

pub type MemoryUsage = sys::FMOD_STUDIO_MEMORY_USAGE;

impl Bus {
    pub unsafe fn new(bus: *mut sys::FMOD_STUDIO_BUS) -> Self {
        Self(bus)
    }

    pub fn is_valid(&self) -> bool {
        let result = unsafe { sys::FMOD_Studio_Bus_IsValid(self.0) };
        if result == sys::FMOD_RESULT::FMOD_OK {
            true
        } else {
            false
        }
    }

    pub fn id(&self) -> Result<GUID> {
        let id: *mut GUID = ptr::null_mut();
        fmod_call!(FMOD_Studio_Bus_GetID, self.0, id => unsafe { *id })
    }

    pub fn path(&self) -> Result<String> {
        let mut len = 0;
        unsafe { sys::FMOD_Studio_Bus_GetPath(self.0, ptr::null_mut(), 0, &mut len) };
        let path = vec![0; len as usize];
        let path_ptr = CString::new(path)?.into_raw();
        fmod_call!(FMOD_Studio_Bus_GetPath, self.0, path_ptr, len, &mut len =>
                   unsafe { CString::from_raw(path_ptr).into_string()? })
    }

    pub fn volume(&self) -> Result<(f32, f32)> {
        let mut volume = 0.0;
        let mut final_volume = 0.0;
        fmod_call!(FMOD_Studio_Bus_GetVolume, self.0, &mut volume, &mut final_volume => (volume, final_volume))
    }

    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        fmod_call!(FMOD_Studio_Bus_SetVolume, self.0, volume => ())
    }

    pub fn paused(&self) -> Result<bool> {
        let mut paused = 0;
        fmod_call!(FMOD_Studio_Bus_GetPaused, self.0, &mut paused => paused % 2 != 0)
    }

    pub fn set_paused(&mut self, paused: bool) -> Result<()> {
        fmod_call!(FMOD_Studio_Bus_SetPaused, self.0, paused as i32 => ())
    }

    pub fn mute(&self) -> Result<bool> {
        let mut mute = 0;
        fmod_call!(FMOD_Studio_Bus_GetMute, self.0, &mut mute => mute % 2 != 0)
    }

    pub fn set_mute(&mut self, mute: bool) -> Result<()> {
        fmod_call!(FMOD_Studio_Bus_SetMute, self.0, mute as i32 => ())
    }

    pub fn stop_all_events(&mut self, mode: i32) -> Result<()> {
        fmod_call!(FMOD_Studio_Bus_StopAllEvents, self.0, mode => ())
    }

    pub fn lock_channel_group(&mut self) -> Result<()> {
        fmod_call!(FMOD_Studio_Bus_LockChannelGroup, self.0 => ())
    }

    pub fn unlock_channel_group(&mut self) -> Result<()> {
        let result = unsafe { sys::FMOD_Studio_Bus_UnlockChannelGroup(self.0) };
        if result == sys::FMOD_RESULT::FMOD_OK {
            Ok(())
        } else {
            Err(anyhow!(result))
        }
    }

    pub fn channel_group(&self) -> Result<Channelgroup> {
        let mut channel_group: *mut sys::FMOD_CHANNELGROUP = ptr::null_mut();
        fmod_call!(FMOD_Studio_Bus_GetChannelGroup, self.0, &mut channel_group => Channelgroup(channel_group))
    }

    pub fn cpu_usage(&self) -> Result<(u32, u32)> {
        let mut exclusive = 0;
        let mut inclusive = 0;
        fmod_call!(FMOD_Studio_Bus_GetCPUUsage, self.0, &mut exclusive, &mut inclusive => (exclusive, inclusive))
    }

    pub fn memory_usage(&self) -> Result<MemoryUsage> {
        let memory_usage: *mut MemoryUsage = ptr::null_mut();
        fmod_call!(FMOD_Studio_Bus_GetMemoryUsage, self.0, memory_usage => unsafe { *memory_usage })
    }
}
