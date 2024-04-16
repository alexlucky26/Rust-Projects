use std::io;
use std::io::BufRead;
use std::net::ToSocketAddrs;
use stp::client::{RequestResult, StpClient};
use stp::error::ConnectResult;

pub struct SocketClient {
    stp: StpClient,
}

impl SocketClient {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    pub fn get_power(&mut self) -> RequestResult {
        let request = "get_power".to_string();
        self.stp.send_request(request)
    }

    pub fn set_on_off(&mut self) -> RequestResult {
        let request = "set_on_off".to_string();
        self.stp.send_request(request)
    }

    fn get_status(&mut self) -> RequestResult {
        let request = "get_status".to_string();
        self.stp.send_request(request)
    }
}
fn main() {
    let addr = get_server_addr();
    let client_conn_result = SocketClient::new(addr);
    match client_conn_result {
        Ok(mut client) => loop {
            println!("Введите номер пункта:\n1. Включить/выключить розетку.\n2. Статус работы розетки.\n3. Инфо о мощности розетки.\nЛюбое иное число. Выход.");
            let mut input_text = String::new();
            let mut stdin = io::stdin().lock();
            stdin
                .read_line(&mut input_text)
                .expect("Ошибка чтения ввода.");
            let trimmed = input_text.trim();
            match trimmed.parse::<u32>() {
                Ok(i) => match i {
                    1 => match client.set_on_off() {
                        Ok(s) => {
                            println!("Розетка имеет статус {}", s);
                        }
                        Err(e) => {
                            eprintln!("Ошибка во время реквеста: {}", e);
                        }
                    },
                    2 => match client.get_status() {
                        Ok(s) => {
                            let split: Vec<&str> = s.split(';').collect();
                            println!("Розетка, находящаяся в комнате `{}` под названием '{}' имеет статус '{}'", split[0], split[1], split[2]);
                        }
                        Err(e) => {
                            eprintln!("Ошибка во время реквеста: {}", e);
                        }
                    },
                    3 => match client.get_power() {
                        Ok(s) => {
                            println!("Мощость розетки = {} А", s);
                        }
                        Err(e) => {
                            eprintln!("Ошибка во время реквеста: {}", e);
                        }
                    },
                    _ => {
                        println!("Завершаю работу.");
                        break;
                    }
                },
                Err(_) => eprintln!(
                    "Ваша строка не является одним из номеров пунктов: {}",
                    trimmed
                ),
            }
        },
        Err(e) => {
            eprintln!("Ошибка при подключении к серверу: {}", e);
        }
    }
}

fn get_server_addr() -> String {
    String::from("127.0.0.1:55331")
}
