// Approach:
// 1. Using collections of trait objects to collect information from devices
// 2. Immutable references to trait object are contained in containers
// 3. Mutable access to the objects from ouside is done by using RefCell'ed fields
// Only some fields are RefCell'ed, just as example

use smarthouse4::devices::{PressureController, Socket, Thermometer};
use smarthouse4::infoprovider::InfoProvider;
use smarthouse4::utils::PowerState;
use smarthouse4::{Room, SmartHouse};

fn main() {
    let kitchen = Room::new(String::from("Кухня"));
    let bedroom = Room::new(String::from("Спальня"));

    let mut house = SmartHouse::new();
    house.add_room(&kitchen);
    house.add_room(&bedroom);

    let socket1 = Socket::new(String::from("Socket 1"), PowerState::On);
    kitchen.add_device(String::from("Socket 1"), &socket1);
    let thermo1 = Thermometer::new(36.6f32);
    kitchen.add_device(String::from("Thermometer 1"), &thermo1);

    socket1.power(PowerState::On);
    let soc_ref = &socket1;
    soc_ref.power(PowerState::Off);

    let socket2 = Socket::new(String::from("Socket 2"), PowerState::On);
    bedroom.add_device(String::from("Socket 2"), &socket2);

    let pc = PressureController::new();
    bedroom.add_device(String::from("Pressure"), &pc);
    println!("{}", house.get_info());

    pc.power(PowerState::On);
    thermo1.set_temperature(38.8);
    socket2.power(PowerState::Off);

    println!("{}", house.get_info());
}
