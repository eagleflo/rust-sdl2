extern crate sdl2;
use sdl2::event::{Event, poll_event};
use sdl2::joystick::Joystick;

fn main() {
    sdl2::init(sdl2::INIT_JOYSTICK);
    let mut joysticks = Vec::new();

    'main : loop {
        match poll_event() {
            Event::Quit(_) => break,
            Event::JoyDeviceAdded(_, idx) => {
                println!("Opening {}", sdl2::joystick::name_for_index(idx).unwrap());
                let joystick = Joystick::from_index(idx);
                match joystick {
                    Ok(j) => {
                        println!("  {} axes", j.num_axes);
                        println!("  {} balls", j.num_balls);
                        println!("  {} buttons", j.num_buttons);
                        println!("  {} hats", j.num_hats);
                        println!("  instance id {}", j.instance_id);
                        println!("  guid {}", sdl2::joystick::get_guid_string(j.get_guid()).unwrap());
                        joysticks.push(j);
                    },
                    Err(e) => println!("Failed to open the joystick: {}", e)
                }
            },
            Event::JoyDeviceRemoved(_, id) => {
                let mut index = 0;
                for (i, joystick) in joysticks.iter().enumerate() {
                    if joystick.instance_id == id {
                        println!("Closing joystick {}", id);
                        joystick.close();
                        index = i;
                    }
                }
                joysticks.remove(index);
            },
            Event::JoyAxisMotion(_, idx, axis, value) => {
                println!("[Joystick {}] Axis {} moved with value {}", idx, axis, value);
            },
            Event::JoyBallMotion(_, idx, ball, xrel, yrel) => {
                println!("[Joystick {}] Ball {} moved ({}, {})", idx, ball, xrel, yrel);
            },
            Event::JoyHatMotion(_, idx, hat, state) => {
                println!("[Joystick {}] Hat {} moved to state {:?}", idx, hat, state);
            },
            Event::JoyButtonDown(_, idx, button) => {
                println!("[Joystick {}] Button down: {}", idx, button);
            },
            Event::JoyButtonUp(_, idx, button) => {
                println!("[Joystick {}] Button up: {}", idx, button);
            },
            _ => continue
        }
    }

    sdl2::quit();
}
