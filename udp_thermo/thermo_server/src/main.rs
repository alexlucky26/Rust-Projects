use std::{io, thread};
use std::net::{UdpSocket, ToSocketAddrs};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;
use rand::Rng;

struct UdpServer {
    socket: UdpSocket,
}
struct Thermo {
    temperature: AtomicI32
}

impl Thermo {
    fn new(temperature: i32) -> Self {
        Self {
            temperature: AtomicI32::new(temperature),
        }
    }
}


impl UdpServer {
    /// Создаёт новый UDP сервер, привязанный к указанному адресу и порту.
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket })
    }

    /// Читает данные из сокета.
    pub fn receive(&self) -> io::Result<(Vec<u8>, std::net::SocketAddr)> {
        let mut buf = [0; 4]; // Буфер для хранения входящих данных.
        let (amt, src) = self.socket.recv_from(&mut buf)?;
        Ok((buf[..amt].to_vec(), src))
    }

    /// Отправляет данные на указанный адрес.
    pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], target: A) -> io::Result<()> {
        self.socket.send_to(buf, target)?;
        Ok(())
    }
}

fn main() {
    let server = UdpServer::bind("127.0.0.1:34254").unwrap_or_else(|e| {
        println!("Ошибка при попытке привязки к адресу: {}", e);
        std::process::exit(1); // Завершаем программу с кодом ошибки 1
    });
    let thermo = Arc::new(Thermo::new(24));

    // Создаем новый поток для изменения температуры
    let thermo_clone = Arc::clone(&thermo);
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        let new_temp = rng.gen_range(30..=40); // Генерируем случайное значение температуры
        println!("Температура изменилась на {}", new_temp);
        thermo_clone.temperature.store(new_temp, Ordering::Release);
        thread::sleep(Duration::from_secs(5)); // Пауза на 5 секунд
    });

    loop {
        match server.receive() {
            Ok((data, src)) => {
                if data == b"temp" {
                    println!("Получен запрос на инфу о температуре от {}", src);
                }
                server.send_to(&thermo.temperature.load(Ordering::Acquire).to_be_bytes(), src).unwrap_or_else(|e| println!("Ошибка при получении данных: {}", e));
            }
            Err(e) => println!("Ошибка при получении данных: {}", e),
        }
    }
}