use rand::{rngs::StdRng, Rng, SeedableRng};
use std::io;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::Mutex;
use tokio::time;

struct UdpServer {
    socket: Mutex<UdpSocket>,
}
struct Thermo {
    temperature: AtomicI32,
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
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = Mutex::new(UdpSocket::bind(addr).await?);
        Ok(Self { socket })
    }

    /// Читает данные из сокета.
    pub async fn receive(&self) -> io::Result<(Vec<u8>, std::net::SocketAddr)> {
        let mut buf = [0; 4]; // Буфер для хранения входящих данных.
        let (amt, src) = self.socket.lock().await.recv_from(&mut buf).await?;
        Ok((buf[..amt].to_vec(), src))
    }

    /// Отправляет данные на указанный адрес.
    pub async fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], target: A) -> io::Result<()> {
        self.socket.lock().await.send_to(buf, target).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let server = UdpServer::bind("127.0.0.1:34254")
        .await
        .unwrap_or_else(|e| {
            println!("Ошибка при попытке привязки к адресу: {}", e);
            std::process::exit(1); // Завершаем программу с кодом ошибки 1
        });
    let thermo = Arc::new(Thermo::new(24));

    // Создаем новый поток для изменения температуры
    let thermo_clone = Arc::clone(&thermo);
    tokio::spawn(async move {
        let mut rng = StdRng::from_entropy(); // Создаем генератор случайных чисел, который реализует `Send`
        loop {
            let new_temp = rng.gen_range(30..=40); // Генерируем случайное значение температуры
            println!("Температура изменилась на {}", new_temp);
            thermo_clone.temperature.store(new_temp, Ordering::SeqCst);
            let duration = Duration::from_secs_f32(5.0);
            time::sleep(duration).await;
        }
    });

    loop {
        match server.receive().await {
            Ok((data, src)) => {
                if data == b"temp" {
                    println!("Получен запрос на инфу о температуре от {}", src);
                }
                server
                    .send_to(
                        &thermo.temperature.load(Ordering::Acquire).to_be_bytes(),
                        src,
                    )
                    .await
                    .unwrap_or_else(|e| println!("Ошибка при получении данных: {}", e));
            }
            Err(e) => println!("Ошибка при получении данных: {}", e),
        }
    }
}
