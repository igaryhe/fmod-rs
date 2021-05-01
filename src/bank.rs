use crate::{bus::Bus, common::GUID, event_description::EventDescription, vca::Vca};
use anyhow::{anyhow, Result};
use core::ptr;
use std::ffi::CString;
use crate::fmod_call;

pub struct Bank(*mut sys::FMOD_STUDIO_BANK);

impl Bank {
    pub fn is_valid(&self) -> bool {
        let result = unsafe { sys::FMOD_Studio_Bank_IsValid(self.0) };
        if result == sys::FMOD_RESULT::FMOD_OK {
            true
        } else {
            false
        }
    }

    pub fn id(&self) -> Result<GUID> {
        let id: *mut GUID = ptr::null_mut();
        fmod_call!(FMOD_Studio_Bank_GetID, self.0, id => unsafe {*id})
    }

    pub fn path(&self) -> Result<String> {
        let mut len = 0;
        unsafe { sys::FMOD_Studio_Bank_GetPath(self.0, ptr::null_mut(), 0, &mut len) };
        let path = vec![0; len as usize];
        let path_ptr = CString::new(path)?.into_raw();
        fmod_call!(FMOD_Studio_Bank_GetPath, self.0, path_ptr, len, &mut len =>
                   unsafe {CString::from_raw(path_ptr).into_string()?})
    }

    pub fn unload(&self) -> Result<()> {
        fmod_call!(FMOD_Studio_Bank_Unload, self.0 => ())
    }

    pub fn load_sample_data(&self) -> Result<()> {
        fmod_call!(FMOD_Studio_Bank_LoadSampleData, self.0 => ())
    }

    pub fn unload_sample_data(&self) -> Result<()> {
        fmod_call!(FMOD_Studio_Bank_UnloadSampleData, self.0 => ())
    }

    pub fn loading_state(&self) -> Result<i32> {
        let mut state = 0;
        fmod_call!(FMOD_Studio_Bank_GetLoadingState, self.0, &mut state => state)
    }

    pub fn sample_loading_state(&self) -> Result<i32> {
        let mut state = 0;
        fmod_call!(FMOD_Studio_Bank_GetSampleLoadingState, self.0, &mut state => state)
    }

    pub fn string_count(&self) -> Result<i32> {
        let mut count = 0;
        fmod_call!(FMOD_Studio_Bank_GetStringCount, self.0, &mut count => count)
    }

    pub fn string_info(&self, index: i32) -> Result<(GUID, String)> {
        let mut len = 0;
        unsafe {
            sys::FMOD_Studio_Bank_GetStringInfo(
                self.0,
                index,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut len,
            )
        };
        let path = vec![0; len as usize];
        let path_ptr = CString::new(path)?.into_raw();
        let id: *mut GUID = ptr::null_mut();
        fmod_call!(FMOD_Studio_Bank_GetStringInfo, self.0, index, id, path_ptr, len, &mut len =>
                   unsafe {(*id, CString::from_raw(path_ptr).into_string()?)})
    }

    pub fn event_count(&self) -> Result<i32> {
        let mut count = 0;
        fmod_call!(FMOD_Studio_Bank_GetEventCount, self.0, &mut count => count)
    }

    pub fn event_list(&self) -> Result<Vec<EventDescription>> {
        let capacity = self.event_count()?;
        let mut list = vec![ptr::null_mut(); capacity as usize];
        let mut count = 0;
        fmod_call!(FMOD_Studio_Bank_GetEventList, self.0, list.as_mut_ptr(), capacity, &mut count =>
                   list.into_iter().map(|i| unsafe { EventDescription::new(i) }).collect())
    }

    pub fn bus_count(&self) -> Result<i32> {
        let mut count = 0;
        fmod_call!(FMOD_Studio_Bank_GetBusCount, self.0, &mut count => count)
    }

    pub fn bus_list(&self) -> Result<Vec<Bus>> {
        let capacity = self.bus_count()?;
        let mut list = vec![ptr::null_mut(); capacity as usize];
        let mut count = 0;
        fmod_call!(FMOD_Studio_Bank_GetBusList, self.0, list.as_mut_ptr(), capacity, &mut count =>
                  list.into_iter().map(|i| unsafe { Bus::new(i) }).collect())
    }

    pub fn vca_count(&self) -> Result<i32> {
        let mut count = 0;
        fmod_call!(FMOD_Studio_Bank_GetVCACount, self.0, &mut count => count)
    }

    pub fn vca_list(&self) -> Result<Vec<Vca>> {
        let capacity = self.vca_count()?;
        let mut list = vec![ptr::null_mut(); capacity as usize];
        let mut count = 0;
        fmod_call!(FMOD_Studio_Bank_GetVCAList, self.0, list.as_mut_ptr(), capacity, &mut count =>
                  list.into_iter().map(|i| unsafe { Vca::new(i) }).collect())
    }

    pub fn user_data(&self) -> Result<Vec<u8>> {
        todo!()
    }

    pub fn set_user_data(&mut self, userdata: &[u8]) -> Result<()> {
        todo!()
    }
}
