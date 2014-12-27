use std::ffi::{c_str_to_bytes, CString};
use std::borrow::ToOwned;
use std::ptr;

use SdlResult;
use get_error;
use event;

pub use sys::joystick as ll;

bitflags! {
    flags HatState: u8 {
        const CENTEREDHATSTATE = 0,
        const UPHATSTATE = 0x01,
        const RIGHTHATSTATE = 0x02,
        const DOWNHATSTATE = 0x04,
        const LEFTHATSTATE = 0x08,
        const RIGHTUPHATSTATE = 0x02 | 0x01,   // RightHatState | UpHatState
        const RIGHTDOWNHATSTATE = 0x02 | 0x04, // RightHatState | DownHatState,
        const LEFTUPHATSTATE = 0x08 | 0x01,    // LeftHatState | UpHatState,
        const LEFTDOWNHATSTATE = 0x08 | 0x04   // LeftHatState | DownHatState
    }
}

pub type JoystickGUID = ll::SDL_JoystickGUID;

#[allow(missing_copy_implementations)]
pub struct Joystick {
    raw: *const ll::SDL_Joystick,
    pub instance_id: i32,
    pub num_axes: i32,
    pub num_balls: i32,
    pub num_buttons: i32,
    pub num_hats: i32
}

impl Joystick {
    pub fn from_ll(raw: *const ll::SDL_Joystick) -> SdlResult<Joystick> {
        unsafe {
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Joystick {
                    raw: raw,
                    instance_id: ll::SDL_JoystickInstanceID(raw),
                    num_axes: ll::SDL_JoystickNumAxes(raw),
                    num_balls: ll::SDL_JoystickNumBalls(raw),
                    num_buttons: ll::SDL_JoystickNumButtons(raw),
                    num_hats: ll::SDL_JoystickNumHats(raw)
                })
            }
        }
    }

    pub fn from_index(device_index: i32) -> SdlResult<Joystick> {
        unsafe {
            let raw = ll::SDL_JoystickOpen(device_index);
            Joystick::from_ll(raw)
        }
    }

    pub fn name(&self) -> SdlResult<String> {
        unsafe {
            let name = ll::SDL_JoystickName(self.raw);
            if name == ptr::null() {
                Err(get_error())
            } else {
                Ok(String::from_utf8_lossy(c_str_to_bytes(&name)).to_string())
            }
        }
    }

    pub fn get_guid(&self) -> JoystickGUID {
        unsafe { ll::SDL_JoystickGetGUID(self.raw) }
    }

    pub fn get_attached(&self) -> bool {
        unsafe { ll::SDL_JoystickGetAttached(self.raw) != 0 }
    }

    pub fn get_axis(&self, axis: i32) -> SdlResult<i16> {
        let val = unsafe { ll::SDL_JoystickGetAxis(self.raw, axis) };
        if val == 0 {
            Err(get_error())
        } else {
            Ok(val)
        }
    }

    pub fn get_hat(&self, hat: i32) -> i8 {
        unsafe { ll::SDL_JoystickGetHat(self.raw, hat) }
    }

    pub fn get_ball(&self, ball: i32) -> SdlResult<(i32, i32)> {
        unsafe {
            let mut xrel = 0;
            let mut yrel = 0;
            if ll::SDL_JoystickGetBall(self.raw, ball, &mut xrel, &mut yrel) < 0 {
                Err(get_error())
            } else {
                Ok((xrel, yrel))
            }
        }
    }

    pub fn get_button(&self, button: i32) -> u8 {
        unsafe { ll::SDL_JoystickGetButton(self.raw, button) }
    }

    pub fn close(&self) {
        unsafe { ll::SDL_JoystickClose(self.raw) }
    }
}

pub fn num_joysticks() -> SdlResult<i32> {
    unsafe {
        let val = ll::SDL_NumJoysticks();
        if val < 0 {
            Err(get_error())
        } else {
            Ok(val)
        }
    }
}

pub fn name_for_index(device_index: i32) -> SdlResult<String> {
    unsafe {
        let name = ll::SDL_JoystickNameForIndex(device_index);
        if name == ptr::null() {
            Err(get_error())
        } else {
            Ok(String::from_utf8_lossy(c_str_to_bytes(&name)).to_string())
        }
    }
}

pub fn get_device_guid(device_index: i32) -> JoystickGUID {
    unsafe { ll::SDL_JoystickGetDeviceGUID(device_index) }
}

pub fn get_guid_string(guid: JoystickGUID) -> Option<String> {
    let buffer = &[0; 33] as *const i8;
    unsafe {
        ll::SDL_JoystickGetGUIDString(guid, buffer, 33);
        if buffer == ptr::null() {
            None
        } else {
            Some(String::from_utf8_lossy(c_str_to_bytes(&buffer)).to_string())
        }
    }
}

pub fn get_guid_from_string(pch_guid: &str) -> JoystickGUID {
    let pch_guid_cstr = CString::from_slice(pch_guid.as_bytes()).as_ptr();
    unsafe { ll::SDL_JoystickGetGUIDFromString(pch_guid_cstr) }
}

pub fn update() {
    unsafe { ll::SDL_JoystickUpdate() }
}

pub fn event_state(state: event::ll::SDL_EventState) -> i32 {
    unsafe { ll::SDL_JoystickEventState(state as i32) }
}
