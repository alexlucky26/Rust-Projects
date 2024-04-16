// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
use super::smart_devices::{SmartDevice, SmartSocket, SmartThermometer};
use thiserror::Error;
pub struct OwningDeviceInfoProvider {
    pub sockets: Vec<SmartSocket>,
}
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub sockets: &'a mut Vec<SmartSocket>,
    pub thermos: &'b mut Vec<SmartThermometer>,
}
#[derive(Error, Debug)]
pub enum DeviceInfoError {
    #[error("Устройство не найдено в Owning Provider: {0}")]
    OwningProviderDeviceNotFound(String),
    #[error("Устройство не найдено в Borrowing Provider: {0}")]
    BorrowingProviderDeviceNotFound(String),
}
pub trait DeviceInfoProvider {
    // метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_info(&self, room: &str, device: &str) -> Result<String, DeviceInfoError>;
    fn delete_device_from_provider(
        &mut self,
        room: &str,
        device: &str,
    ) -> Result<bool, DeviceInfoError>;
    fn delete_all_devices_for_room(&mut self, room: &str);
}
impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Result<String, DeviceInfoError> {
        self.sockets
            .iter()
            .find(|s| s.name.0 == room && s.name.1 == device)
            .map(|d| {
                format!(
                    "Устройство '{}' в комнате'{}' имеет статус: {}\n",
                    d.name.1,
                    d.name.0,
                    d.get_state()
                )
            })
            .ok_or(DeviceInfoError::OwningProviderDeviceNotFound(format!(
                "Device:{}/Room:{}\n",
                device, room
            )))
    }

    fn delete_device_from_provider(
        &mut self,
        room: &str,
        device: &str,
    ) -> Result<bool, DeviceInfoError> {
        match self.sockets.iter().position(|s| s.name.0 == room && s.name.1 == device) {
            Some(d) => { self.sockets.remove(d); Ok(true) },
            None => Err(DeviceInfoError::OwningProviderDeviceNotFound(
                format!("Ваше устройство {} находящееся в комнате под названием {} не было найдено у провайдера!\n", device, room)
            ))
        }
    }

    // В случае удаления комнаты, удалим всю информацию о девайсах из провайдера
    fn delete_all_devices_for_room(&mut self, room: &str) {
        self.sockets.retain(|s| s.name.0 != room);
    }
}

impl OwningDeviceInfoProvider {
    pub fn new() -> Self {
        Self {
            sockets: Vec::new(),
        }
    }
}

impl Default for OwningDeviceInfoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_info(&self, room: &str, device: &str) -> Result<String, DeviceInfoError> {
        match self.sockets.iter().find(|s| s.name.0 == room && s.name.1 == device) {
            Some(d) => Ok(format!("Ваше устройство {} находящееся в комнате под названием {} имеет рабочий статус: {} \n", d.name.1, d.name.0, d.get_state())),
            None => {
                match self.thermos.iter().find(|s| s.name.0 == room && s.name.1 == device) {
                    Some(d) => Ok(format!("Ваше устройство {} находящееся в комнате под названием {} имеет рабочий статус: {} \n", d.name.1, d.name.0, d.get_state())),
                    None => Err(DeviceInfoError::BorrowingProviderDeviceNotFound(
                            format!("Device: {}/ Room: {}\n", device, room)
                    ))
                }
            }
        }
    }

    fn delete_device_from_provider(
        &mut self,
        room: &str,
        device: &str,
    ) -> Result<bool, DeviceInfoError> {
        match self
            .sockets
            .iter()
            .position(|s| s.name.0 == room && s.name.1 == device)
        {
            Some(d) => {
                self.sockets.remove(d);
                Ok(true)
            }
            None => {
                match self
                    .thermos
                    .iter()
                    .position(|s| s.name.0 == room && s.name.1 == device)
                {
                    Some(d) => {
                        self.thermos.remove(d);
                        Ok(true)
                    }
                    None => Err(DeviceInfoError::BorrowingProviderDeviceNotFound(format!(
                        "Device: {}/ Room: {}\n",
                        device, room
                    ))),
                }
            }
        }
    }

    // В случае удаления комнаты, удалим всю информацию о девайсах из провайдера
    fn delete_all_devices_for_room(&mut self, room: &str) {
        self.sockets.retain(|s| s.name.0 != room);
        self.thermos.retain(|t| t.name.0 != room);
    }
}

impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
    pub fn new(sockets: &'a mut Vec<SmartSocket>, thermos: &'b mut Vec<SmartThermometer>) -> Self {
        Self { sockets, thermos }
    }
}
