use crate::devices::device_info_providers::{DeviceInfoProvider, OwningDeviceInfoProvider};
use crate::devices::smart_devices::SmartSocket;
use crate::smart_house::SmartHouse;
#[test]
fn test_smart_house_initialization() {
    let house = SmartHouse::new();
    assert!(house.rooms.contains_key("Kitchen"));
    assert!(house.rooms.contains_key("Bedroom"));
}

#[test]
fn test_get_rooms() {
    let house = SmartHouse::new();
    let rooms = house.get_rooms();
    assert_eq!(rooms.len(), 2);
    assert!(rooms.contains(&"Kitchen"));
    assert!(rooms.contains(&"Bedroom"));
}

#[test]
fn test_devices_in_room() {
    let house = SmartHouse::new();
    let kitchen_devices = house.devices("Kitchen");
    assert!(kitchen_devices.contains(&"Smart Socket 35"));
    assert!(kitchen_devices.contains(&"Smart Thermometer 113"));

    let bedroom_devices = house.devices("Bedroom");
    assert!(bedroom_devices.contains(&"Smart Socket 51"));
    assert!(bedroom_devices.contains(&"Smart Socket 99"));
}

#[test]
fn test_device_info_provider() {
    let socket = SmartSocket {
        state: true,
        name: ("Kitchen".to_string(), "Smart Socket 35".to_string()),
    };
    let mut info_provider = OwningDeviceInfoProvider::new();
    info_provider.sockets.push(socket);
    let info = info_provider.get_device_info("Kitchen", "Smart Socket 35");
    assert!(info.contains("Socket State: true"));
}

#[test]
fn test_create_report() {
    let house = SmartHouse::new();
    let socket = SmartSocket {
        state: true,
        name: ("Kitchen".to_string(), "Smart Socket 35".to_string()),
    };
    let mut info_provider = OwningDeviceInfoProvider::new();
    info_provider.sockets.push(socket);
    let report = house.create_report(&info_provider);
    assert!(report.contains("Socket State: true"));
}
