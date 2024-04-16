use crate::devices::device_info_providers::{DeviceInfoError, DeviceInfoProvider};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct SmartHouse {
    pub rooms: HashMap<String, Room>,
}
#[derive(Debug)]
pub struct Room {
    pub devices: HashSet<String>,
}
#[derive(Debug)]
pub enum SmartHouseError {
    RoomIsNotFound,
    RoomIsAlreadyInHouse,
    EmptyRooms,
    DeviceIsNotFoundInRoom,
    DeviceIsAlreadyInRoom,
}

#[derive(Debug)]
pub enum SmartHouseReportErrors {
    SmartHouseComponentsError(SmartHouseError),
    ProvidersInfoError(DeviceInfoError),
}
impl Display for SmartHouseReportErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Smart House errors occured!")
    }
}
impl Error for SmartHouseReportErrors {}

#[derive(Debug)]
pub struct GlobalError {
    source: SmartHouseReportErrors,
}

impl fmt::Display for GlobalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            SmartHouseReportErrors::SmartHouseComponentsError(e) => match e {
                SmartHouseError::DeviceIsNotFoundInRoom => {
                    write!(f, "One or more devices wasn't/weren't found!")
                }
                _ => write!(f, "There are invalid or empty rooms!"),
            },
            _ => write!(f, "Some of providers reported an error about devices."),
        }
    }
}

impl Error for GlobalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl From<SmartHouseError> for SmartHouseReportErrors {
    fn from(e: SmartHouseError) -> Self {
        SmartHouseReportErrors::SmartHouseComponentsError(e)
    }
}

impl SmartHouse {
    pub fn new() -> Self {
        let mut rooms: HashMap<String, Room> = HashMap::new();

        let mut hash = HashSet::new();
        hash.insert("Smart Socket 35".to_string());
        hash.insert("Smart Thermometer 113".to_string());
        rooms.insert("Kitchen".to_string(), Room { devices: hash });

        let mut hash2 = HashSet::new();
        hash2.insert("Smart Socket 99".to_string());
        hash2.insert("Smart Socket 51".to_string());
        rooms.insert("Bedroom".to_string(), Room { devices: hash2 });

        Self { rooms }
    }

    pub fn get_rooms(&self) -> Option<Vec<&str>> {
        let rooms_map = self.rooms.iter().map(|r| r.0.as_str());
        if rooms_map.len() == 0 {
            return None;
        }
        Some(rooms_map.collect())
    }

    pub fn add_room(&mut self, room_name: String) -> Result<bool, SmartHouseError> {
        let room = self
            .rooms
            .iter()
            .map(|r| r.0.as_str())
            .find(|r| *r == room_name);
        match room {
            Some(_) => Err(SmartHouseError::RoomIsAlreadyInHouse),
            None => {
                self.rooms.insert(
                    room_name,
                    Room {
                        devices: HashSet::new(),
                    },
                );
                Ok(true)
            }
        }
    }

    pub fn remove_room(&mut self, room_name: String) -> Result<bool, SmartHouseError> {
        let rooms = self.rooms.iter().find(|r| r.0.as_str() == room_name);
        match rooms {
            Some(_) => {
                self.rooms.remove(&room_name);
                Ok(true)
            }
            None => Err(SmartHouseError::RoomIsNotFound),
        }
    }

    pub fn remove_device_from_room(
        &mut self,
        room: &str,
        device: &str,
    ) -> Result<bool, SmartHouseError> {
        let rooms = self.rooms.iter_mut().find(|r| r.0.as_str() == room);
        match rooms {
            Some(n) => {
                let devices_in_room = &mut n.1.devices;
                let current_device = devices_in_room
                    .iter()
                    .map(|d| d.as_str())
                    .find(|d| d == &device);
                match current_device {
                    Some(_) => {
                        devices_in_room.remove(device);
                        Ok(true)
                    }
                    None => Err(SmartHouseError::DeviceIsNotFoundInRoom),
                }
            }
            None => Err(SmartHouseError::RoomIsNotFound), // конкретная комната с девайсами не была найдена
        }
    }
    pub fn add_device_to_room(
        &mut self,
        room: &str,
        device: &str,
    ) -> Result<bool, SmartHouseError> {
        let rooms = self.rooms.iter_mut().find(|r| r.0.as_str() == room);
        return match rooms {
            Some(n) => {
                let devices_in_room = &mut n.1.devices;
                let current_device = devices_in_room
                    .iter()
                    .map(|d| d.as_str())
                    .find(|d| d == &device);
                match current_device {
                    Some(_) => Err(SmartHouseError::DeviceIsAlreadyInRoom),
                    None => {
                        devices_in_room.insert(device.parse().unwrap());
                        Ok(true)
                    }
                }
            }
            None => Err(SmartHouseError::RoomIsNotFound), // конкретная комната с девайсами не была найдена
        };
    }

    pub fn devices(&self, room: &str) -> Result<Vec<&str>, SmartHouseError> {
        let rooms = self.rooms.iter().find(|r| r.0.as_str() == room);
        return match rooms {
            Some(n) => {
                let devices_in_room = n.1.devices.iter().map(|d| d.as_str());
                if devices_in_room.len() == 0 {
                    return Err(SmartHouseError::DeviceIsNotFoundInRoom);
                }
                Ok(devices_in_room.collect())
            }
            None => Err(SmartHouseError::RoomIsNotFound), // конкретная комната с девайсами не была найдена
        };
    }

    pub fn create_report<T: DeviceInfoProvider>(
        &self,
        device_info_provider: &T,
    ) -> Result<String, GlobalError> {
        let mut report = String::new();
        match self.get_rooms() {
            Some(r) => {
                for room in r.iter() {
                    match self.devices(room) {
                        Ok(device) => {
                            for d in device.iter() {
                                match device_info_provider.get_device_info(room, d) {
                                    Ok(info) => report.push_str(info.as_str()),
                                    Err(e) => match e {
                                        DeviceInfoError::OwningProviderDeviceNotFound(
                                            description,
                                        ) => {
                                            report.push_str(description.as_str());
                                        }
                                        DeviceInfoError::BorrowingProviderDeviceNotFound(
                                            description,
                                        ) => {
                                            report.push_str(description.as_str());
                                        }
                                    },
                                }
                            }
                        }
                        Err(_) => {
                            println!("Устройства не найдены провайдером в комнате {}", room);
                            /*return Err(GlobalError {
                                source: SmartHouseReportErrors::from(e),
                            })*/
                        }
                    }
                }
            }
            None => {
                return Err(GlobalError {
                    source: SmartHouseReportErrors::from(SmartHouseError::EmptyRooms),
                })
            }
        }
        Ok(report)
    }
}
