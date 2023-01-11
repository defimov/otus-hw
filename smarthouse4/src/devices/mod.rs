use crate::{infoprovider::InfoProvider, utils::PowerState};
use std::cell::RefCell;

pub struct Socket {
    name: String,
    /// Using RefCell'ed field to avoid borrow checker's complains
    power: RefCell<PowerState>,
}

impl Socket {
    pub fn new(name: String, power: PowerState) -> Socket {
        Socket {
            name,
            power: RefCell::new(power),
        }
    }

    // RefCell and immutable self
    pub fn power(&self, power: PowerState) {
        self.power.replace(power);
    }
}

impl InfoProvider for Socket {
    fn get_info(&self) -> String {
        format!("{}, power is {:?}", self.name, self.power.borrow())
    }
}

pub struct Thermometer {
    temperature: RefCell<f32>,
}

impl Thermometer {
    pub fn new(temperature: f32) -> Thermometer {
        Thermometer {
            temperature: RefCell::new(temperature),
        }
    }

    pub fn set_temperature(&self, t: f32) {
        self.temperature.replace(t);
    }
}

impl InfoProvider for Thermometer {
    fn get_info(&self) -> String {
        format!("Thermometer, {}", self.temperature.borrow())
    }
}

pub struct PressureController {
    pressure: f32,
    power: RefCell<PowerState>,
}

impl PressureController {
    pub fn new() -> PressureController {
        PressureController {
            pressure: 0f32,
            power: RefCell::new(PowerState::Off),
        }
    }

    pub fn get_pressure(&self) -> f32 {
        self.pressure
    }

    pub fn power(&self, power: PowerState) {
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

impl Default for PressureController {
    fn default() -> Self {
        Self::new()
    }
}
