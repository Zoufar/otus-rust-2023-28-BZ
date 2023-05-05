pub enum Device<'a> {
    Socket(&'a SmartSocket),
    Thermo(&'a SmartThermometer),
}

// Пользовательские устройства:
pub struct SmartSocket {}

pub struct SmartThermometer {}
