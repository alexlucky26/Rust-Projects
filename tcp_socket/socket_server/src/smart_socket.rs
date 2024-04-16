use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct SmartSocket {
    pub is_on: AtomicBool,
    pub power: AtomicI32,
    pub name: Arc<Mutex<String>>,
    pub room: Arc<Mutex<String>>,
}

impl Clone for SmartSocket {
    fn clone(&self) -> Self {
        SmartSocket {
            is_on: AtomicBool::new(self.is_on.load(Ordering::Acquire)),
            power: AtomicI32::new(self.power.load(Ordering::Acquire)),
            name: Arc::clone(&self.name),
            room: Arc::clone(&self.room),
        }
    }
}

impl SmartSocket {
    pub fn new(is_on: bool, power: i32, name: String, room: String) -> Self {
        SmartSocket {
            is_on: AtomicBool::new(is_on),
            power: AtomicI32::new(power),
            name: Arc::new(Mutex::new(name)),
            room: Arc::new(Mutex::new(room)),
        }
    }
}
