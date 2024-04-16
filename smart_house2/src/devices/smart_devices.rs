// Пользовательские устройства:
#[derive(Clone)]
pub struct SmartSocket {
    pub state: bool,
    pub name: (String, String),
}

pub struct SmartThermometer {
    pub temperature: i32,
    pub name: (String, String),
}
pub trait SmartDevice {
    fn get_state(&self) -> String;
}
impl SmartDevice for SmartSocket {
    fn get_state(&self) -> String {
        format!("Socket State: {}", self.state)
    }
}
impl SmartDevice for SmartThermometer {
    fn get_state(&self) -> String {
        format!("Temperature: {}°C", self.temperature)
    }
}
