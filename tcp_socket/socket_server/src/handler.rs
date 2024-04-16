use crate::smart_socket::SmartSocket;
use std::ops::Not;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub struct Request<'a>(&'a str);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn get_data(&mut self) -> &'a str {
        self.0
    }
}

pub struct RequestHandler {
    socket: Arc<SmartSocket>,
}

impl RequestHandler {
    pub fn new(socket: Arc<SmartSocket>) -> Self {
        Self { socket }
    }

    pub fn handle(&mut self, mut request: Request) -> String {
        let command = request.get_data();
        match command {
            "set_on_off" => self.set_on_off(),
            "get_power" => self.get_power(),
            "get_status" => self.get_status(),
            _ => "Bad command".into(),
        }
    }

    fn set_on_off(&mut self) -> String {
        let is_on = self.socket.is_on.load(Ordering::Acquire);
        self.socket.is_on.store(is_on.not(), Ordering::Release);
        String::from(if self.socket.is_on.load(Ordering::Acquire) {
            "вкл."
        } else {
            "выкл."
        })
    }

    fn get_power(&self) -> String {
        self.socket.power.load(Ordering::Acquire).to_string()
    }

    fn get_status(&self) -> String {
        format!(
            "{};{};{}",
            self.socket.room.lock().unwrap(),
            self.socket.name.lock().unwrap(),
            if self.socket.is_on.load(Ordering::Acquire) {
                "вкл."
            } else {
                "выкл."
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_set_on_off() {
        let socket = Arc::new(SmartSocket::new(
            true,
            0,
            "Test".to_string(),
            "Room".to_string(),
        ));
        let mut handler = RequestHandler::new(Arc::clone(&socket));

        assert_eq!(handler.handle(Request::new("set_on_off")), "выкл.");
        assert_eq!(handler.handle(Request::new("set_on_off")), "вкл.");
    }

    #[test]
    fn test_get_power() {
        let power = 100;
        let socket = Arc::new(SmartSocket::new(
            false,
            power,
            "Test".to_string(),
            "Room".to_string(),
        ));
        let mut handler = RequestHandler::new(Arc::clone(&socket));

        assert_eq!(handler.handle(Request::new("get_power")), power.to_string());
    }

    #[test]
    fn test_get_status() {
        let socket = Arc::new(SmartSocket::new(
            true,
            0,
            "Socket_1".to_string(),
            "Kitchen".to_string(),
        ));
        let mut handler = RequestHandler::new(Arc::clone(&socket));

        let expected_status = format!("{};{};{}", "Kitchen", "Socket_1", "вкл.");
        assert_eq!(handler.handle(Request::new("get_status")), expected_status);
    }
}
