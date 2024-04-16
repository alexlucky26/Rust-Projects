// Пример как использовать SmartHouse.
// Библиотека предоставляет структуру дома в комнатах которого расположены устройства.
use smart_house2::devices::device_info_providers::OwningDeviceInfoProvider;
use smart_house2::devices::smart_devices::SmartSocket; // Импортируйте устройство
use smart_house2::smart_house::SmartHouse; // Импортируйте SmartHouse // Импортируйте трейт DeviceInfoProvider
fn main() {
    // Создайте экземпляр SmartHouse
    let house = SmartHouse::new();

    // Создайте экземпляр провайдера информации об устройствах
    let socket = SmartSocket {
        state: true,
        name: ("Kitchen".to_string(), "Smart Socket 35".to_string()),
    };
    let info_provider = OwningDeviceInfoProvider {
        sockets: vec![socket],
    };

    // Сгенерируйте отчет о состоянии умного дома
    let report = house.create_report(&info_provider);

    // Выведите отчет на экран
    println!("Отчет о состоянии умного дома:\n{:?}", report);
}
