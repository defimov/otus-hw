pub mod devices;
pub mod infoprovider;
pub mod utils;

use crate::infoprovider::InfoProvider;

pub struct SmartHouse<'a> {
    rooms: Vec<&'a Room<'a>>,
}

impl<'a> SmartHouse<'a> {
    pub fn new() -> SmartHouse<'a> {
        SmartHouse { rooms: Vec::new() }
    }

    pub fn add_room(&mut self, room: &'a Room<'a>) -> &mut Self {
        self.rooms.push(room);
        self
    }
}

impl<'a> Default for SmartHouse<'a> {
    fn default() -> Self {
        Self::new()
    }
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

use std::cell::RefCell;
use std::collections::HashMap;

pub struct Room<'a> {
    name: String,
    devices: RefCell<HashMap<String, &'a dyn InfoProvider>>,
}

impl<'a> Room<'a> {
    pub fn new(name: String) -> Room<'a> {
        Room {
            name,
            devices: RefCell::new(HashMap::<String, &dyn InfoProvider>::new()),
        }
    }

    pub fn add_device(&self, device_name: String, device: &'a dyn InfoProvider) -> &Self {
        self.devices
            .borrow_mut()
            .entry(device_name)
            .or_insert(device);
            self
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
