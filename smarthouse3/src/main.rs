// Approach:
// 1. Using collections of trait objects to collect information from devices
// 2. Immutable references to trait object are contained in containers
// 3. Mutable access to the objects from ouside is done by using RefCell'ed fields
// Only some fields are RefCell'ed, just as example

use std::{cell::RefCell, collections::HashMap};

// General info provider for both devices and device containers (rooms)
trait InfoProvider {
    fn get_info(&self) -> String;
}

// Just an example
#[derive(Debug)]
pub enum PowerState {
    Off,
    On,
}

// Simple SmartSocket
struct Socket {
    name: String,
    /// Using RefCell'ed field to avoid borrow checker's complains
    power: RefCell<PowerState>,
}

impl Socket {
    fn new(name: String, power: PowerState) -> Socket {
        Socket {
            name,
            power: RefCell::new(power),
        }
    }

    // RefCell and immutable self
    fn power(&self, power: PowerState) {
        self.power.replace(power);
    }
}

impl InfoProvider for Socket {
    fn get_info(&self) -> String {
        format!("{}, power is {:?}", self.name, self.power.borrow())
    }
}

struct Thermometer {
    temperature: RefCell<f32>,
}

impl Thermometer {
    fn new(temperature: f32) -> Thermometer {
        Thermometer {
            temperature: RefCell::new(temperature),
        }
    }

    fn set_temperature(&self, t: f32) {
        self.temperature.replace(t);
    }
}

impl InfoProvider for Thermometer {
    fn get_info(&self) -> String {
        format!("Thermometer, {}", self.temperature.borrow())
    }
}

struct PressureController {
    pressure: f32,
    power: RefCell<PowerState>,
}

impl PressureController {
    fn new() -> PressureController {
        PressureController {
            pressure: 0f32,
            power: RefCell::new(PowerState::Off),
        }
    }

    fn get_pressure(&self) -> f32 {
        self.pressure
    }

    fn power(&self, power: PowerState) {
        self.power.replace(power);
    }
}

impl InfoProvider for PressureController {
    fn get_info(&self) -> String {
        format!(
            "Pressure = {}, power is {:?}",
            self.get_pressure(),
            self.power.borrow()
        )
    }
}

struct Room<'a> {
    name: String,
    devices: RefCell<HashMap<String, &'a dyn InfoProvider>>,
}

impl<'a> Room<'a> {
    fn new(name: String) -> Room<'a> {
        Room {
            name,
            devices: RefCell::new(HashMap::<String, &dyn InfoProvider>::new()),
        }
    }

    fn add_device(&self, device_name: String, device: &'a dyn InfoProvider) {
        self.devices
            .borrow_mut()
            .entry(device_name)
            .or_insert(device);
    }
}

impl<'a> InfoProvider for Room<'a> {
    fn get_info(&self) -> String {
        let mut s = format!("{}{}", self.name, "\n");

        for device in self.devices.borrow().values() {
            s.push_str(device.get_info().as_str());
            s.push('\n');
        }
        s
    }
}

struct SmartHouse<'a> {
    rooms: Vec<&'a Room<'a>>,
}

impl<'a> InfoProvider for SmartHouse<'a> {
    fn get_info(&self) -> String {
        let mut res = String::from("***** Отчет *****\n");
        for room in &self.rooms {
            res.push_str((*room).get_info().as_str());
        }
        res
    }
}

impl<'a> SmartHouse<'a> {
    fn new() -> SmartHouse<'a> {
        SmartHouse { rooms: Vec::new() }
    }

    fn add_room(&mut self, room: &'a Room<'a>) {
        self.rooms.push(room);
    }
}
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
