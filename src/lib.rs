#[macro_use]
extern crate bitflags;

extern crate fmod_sys as sys;

pub mod common;
pub mod system;
pub mod error;
pub mod bus;
pub mod vca;
pub mod bank;
pub mod command_replay;
pub mod event_description;
pub mod event_instance;

#[macro_export]
macro_rules! fmod_call {
    ($func:ident, $($param:expr),* => $val:expr) => {
        {
            let result = unsafe { sys::$func($($param,)*) };
            if result == sys::FMOD_RESULT::FMOD_OK {
                Ok($val)
            } else {
                Err(anyhow!(result))
            }
        }
    }
}
