extern crate sdl2;
use sdl2::event::{Event, poll_event};
use sdl2::controller::Controller;

fn main() {
    sdl2::init(sdl2::INIT_GAME_CONTROLLER);
    let mut controllers = Vec::new();

    'main : loop {
        match poll_event() {
            Event::Quit(_) => break,
            Event::ControllerDeviceAdded(_, idx) => {
                println!("Opening {}", sdl2::controller::name_for_index(idx).unwrap());
                let controller = Controller::from_index(idx);
                match controller {
                    Ok(c) => controllers.push(c),
                    Err(e) => println!("Failed to open the controller: {}", e)
                }
            },
            Event::ControllerDeviceRemoved(_, id) => {
                let mut index = 0;
                for (i, controller) in controllers.iter().enumerate() {
                    let joystick = controller.get_joystick();
                    if joystick.unwrap().instance_id == id {
                        println!("Closing controller {}", id);
                        controller.close();
                        index = i;
                    }
                }
                controllers.remove(index);
            },
            Event::ControllerButtonDown(_, idx, button) => {
                println!("[Controller {}] Button down: {:?}", idx, button);
            },
            Event::ControllerButtonUp(_, idx, button) => {
                println!("[Controller {}] Button up: {:?}", idx, button);
            },
            Event::ControllerAxisMotion(_, idx, axis, value) => {
                println!("[Controller {}] Axis {:?} moved with value {}", idx, axis, value)
            },
            _ => continue
        }
    }

    sdl2::quit();
}
