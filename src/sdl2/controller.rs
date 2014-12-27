use libc::c_int;
use std::ffi::{c_str_to_bytes, CString};
use std::borrow::ToOwned;
use std::ptr;

use SdlResult;
use get_error;
use event;
use joystick::{Joystick, JoystickGUID};

pub use sys::controller as ll;

#[derive(Copy, Clone, PartialEq)]
#[repr(i32)]
pub enum ControllerAxis {
    Invalid      = ll::SDL_CONTROLLER_AXIS_INVALID,
    LeftX        = ll::SDL_CONTROLLER_AXIS_LEFTX,
    LeftY        = ll::SDL_CONTROLLER_AXIS_LEFTY,
    RightX       = ll::SDL_CONTROLLER_AXIS_RIGHTX,
    RightY       = ll::SDL_CONTROLLER_AXIS_RIGHTY,
    TriggerLeft  = ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT,
    TriggerRight = ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
}

pub fn wrap_controller_axis(bitflags: u8) -> ControllerAxis {
    match bitflags as c_int {
        ll::SDL_CONTROLLER_AXIS_LEFTX        => ControllerAxis::LeftX,
        ll::SDL_CONTROLLER_AXIS_LEFTY        => ControllerAxis::LeftY,
        ll::SDL_CONTROLLER_AXIS_RIGHTX       => ControllerAxis::RightX,
        ll::SDL_CONTROLLER_AXIS_RIGHTY       => ControllerAxis::RightY,
        ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT  => ControllerAxis::TriggerLeft,
        ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT => ControllerAxis::TriggerRight,
        _ => panic!("unhandled controller axis")
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(i32)]
pub enum ControllerButton {
    Invalid       = ll::SDL_CONTROLLER_BUTTON_INVALID,
    A             = ll::SDL_CONTROLLER_BUTTON_A,
    B             = ll::SDL_CONTROLLER_BUTTON_B,
    X             = ll::SDL_CONTROLLER_BUTTON_X,
    Y             = ll::SDL_CONTROLLER_BUTTON_Y,
    Back          = ll::SDL_CONTROLLER_BUTTON_BACK,
    Guide         = ll::SDL_CONTROLLER_BUTTON_GUIDE,
    Start         = ll::SDL_CONTROLLER_BUTTON_START,
    LeftStick     = ll::SDL_CONTROLLER_BUTTON_LEFTSTICK,
    RightStick    = ll::SDL_CONTROLLER_BUTTON_RIGHTSTICK,
    LeftShoulder  = ll::SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
    RightShoulder = ll::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
    DPadUp        = ll::SDL_CONTROLLER_BUTTON_DPAD_UP,
    DPadDown      = ll::SDL_CONTROLLER_BUTTON_DPAD_DOWN,
    DPadLeft      = ll::SDL_CONTROLLER_BUTTON_DPAD_LEFT,
    DPadRight     = ll::SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
}

pub fn wrap_controller_button(bitflags: u8) -> ControllerButton {
    match bitflags as c_int {
        ll::SDL_CONTROLLER_BUTTON_A             => ControllerButton::A,
        ll::SDL_CONTROLLER_BUTTON_B             => ControllerButton::B,
        ll::SDL_CONTROLLER_BUTTON_X             => ControllerButton::X,
        ll::SDL_CONTROLLER_BUTTON_Y             => ControllerButton::Y,
        ll::SDL_CONTROLLER_BUTTON_BACK          => ControllerButton::Back,
        ll::SDL_CONTROLLER_BUTTON_GUIDE         => ControllerButton::Guide,
        ll::SDL_CONTROLLER_BUTTON_START         => ControllerButton::Start,
        ll::SDL_CONTROLLER_BUTTON_LEFTSTICK     => ControllerButton::LeftStick,
        ll::SDL_CONTROLLER_BUTTON_RIGHTSTICK    => ControllerButton::RightStick,
        ll::SDL_CONTROLLER_BUTTON_LEFTSHOULDER  => ControllerButton::LeftShoulder,
        ll::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => ControllerButton::RightShoulder,
        ll::SDL_CONTROLLER_BUTTON_DPAD_UP       => ControllerButton::DPadUp,
        ll::SDL_CONTROLLER_BUTTON_DPAD_DOWN     => ControllerButton::DPadDown,
        ll::SDL_CONTROLLER_BUTTON_DPAD_LEFT     => ControllerButton::DPadLeft,
        ll::SDL_CONTROLLER_BUTTON_DPAD_RIGHT    => ControllerButton::DPadRight,
        _ => panic!("unhandled controller button")
    }
}

#[allow(missing_copy_implementations)]
pub struct Controller {
    raw: *const ll::SDL_GameController,
}

impl Controller {
    pub fn from_index(joystick_index: i32) -> SdlResult<Controller> {
        let raw = unsafe { ll::SDL_GameControllerOpen(joystick_index) };
        if raw == ptr::null() {
            Err(get_error())
        } else {
            Ok(Controller { raw: raw })
        }
    }

    pub fn name(&self) -> Option<String> {
        unsafe {
            let name = ll::SDL_GameControllerName(self.raw);
            if name == ptr::null() {
                None
            } else {
                Some(String::from_utf8_lossy(c_str_to_bytes(&name)).to_string())
            }
        }
    }

    pub fn get_attached(&self) -> bool {
        unsafe { ll::SDL_GameControllerGetAttached(self.raw) != 0 }
    }

    pub fn get_joystick(&self) -> SdlResult<Joystick> {
        Joystick::from_ll(self.raw)
    }

    pub fn get_axis(&self, axis: ControllerAxis) -> i16 {
        unsafe { ll::SDL_GameControllerGetAxis(self.raw, axis as i32) }
    }

    pub fn get_bind_for_axis(&self, axis: ControllerAxis) -> ll::SDL_GameControllerButtonBind {
        unsafe { ll::SDL_GameControllerGetBindForAxis(self.raw, axis as i32) }
    }

    pub fn get_button(&self, button: ControllerButton) -> u8 {
        unsafe { ll::SDL_GameControllerGetButton(self.raw, button as i32) }
    }

    pub fn get_bind_for_button(&self, button: ControllerButton) -> ll::SDL_GameControllerButtonBind {
        unsafe { ll::SDL_GameControllerGetBindForButton(self.raw, button as i32) }
    }

    pub fn close(&self) {
        unsafe { ll::SDL_GameControllerClose(self.raw) }
    }

    pub fn mapping(&self) -> Option<String> {
        unsafe {
            let mapping = ll::SDL_GameControllerMapping(self.raw);
            if mapping == ptr::null() {
                None
            } else {
                Some(String::from_utf8_lossy(c_str_to_bytes(&mapping)).to_string())
            }
        }
    }
}

pub fn add_mapping(mapping: &str) -> i32 {
    let mapping_cstr = CString::from_slice(mapping.as_bytes()).as_ptr();
    unsafe { ll::SDL_GameControllerAddMapping(mapping_cstr) }
}

pub fn mapping_for_guid(guid: JoystickGUID) -> SdlResult<String> {
    unsafe {
        let mapping = ll::SDL_GameControllerMappingForGUID(guid);
        if mapping == ptr::null() {
            Err(get_error())
        } else {
            Ok(String::from_utf8_lossy(c_str_to_bytes(&mapping)).to_string())
        }
    }
}

pub fn is_game_controller(joystick_index: i32) -> bool {
    unsafe { ll::SDL_IsGameController(joystick_index) != 0 }
}

pub fn name_for_index(joystick_index: i32) -> Option<String> {
    unsafe {
        let name = ll::SDL_GameControllerNameForIndex(joystick_index);
        if name == ptr::null() {
            None
        } else {
            Some(String::from_utf8_lossy(c_str_to_bytes(&name)).to_string())
        }
    }
}

pub fn event_state(state: event::ll::SDL_EventState) -> i32 {
    unsafe { ll::SDL_GameControllerEventState(state as i32) }
}

pub fn update() {
    unsafe { ll::SDL_GameControllerUpdate() }
}

pub fn get_axis_from_string(pch_string: &str) -> ControllerAxis {
    let pch_string_cstr = CString::from_slice(pch_string.as_bytes()).as_ptr();
    unsafe { wrap_controller_axis(ll::SDL_GameControllerGetAxisFromString(pch_string_cstr) as u8) }
}

pub fn get_string_for_axis(axis: ControllerAxis) -> Option<String> {
    unsafe {
        let string = ll::SDL_GameControllerGetStringForAxis(axis as i32);
        if string == ptr::null() {
            None
        } else {
            Some(String::from_utf8_lossy(c_str_to_bytes(&string)).to_string())
        }
    }
}

pub fn get_button_from_string(pch_string: &str) -> ControllerButton {
    let pch_string_cstr = CString::from_slice(pch_string.as_bytes()).as_ptr();
    unsafe { wrap_controller_button(ll::SDL_GameControllerGetButtonFromString(pch_string_cstr) as u8) }
}

pub fn get_string_for_button(button: ControllerButton) -> Option<String> {
    unsafe {
        let string = ll::SDL_GameControllerGetStringForButton(button as i32);
        if string == ptr::null() {
            None
        } else {
            Some(String::from_utf8_lossy(c_str_to_bytes(&string)).to_string())
        }
    }
}
