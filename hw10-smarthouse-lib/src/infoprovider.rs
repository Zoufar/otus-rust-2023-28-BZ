use crate::device::Device;
use crate::device::SmartSocket;
use crate::device::SmartThermometer;
use crate::smarthouse::SmartHouse;
use std::ptr;

pub trait DeviceInfoProvider {
    fn device_report(&self, room: &str, device: &str) -> String;
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
}

pub struct OwningDeviceInfoProvider<'a> {
    pub house: SmartHouse<'a>,
    pub socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider<'_> {
    fn device_report(&self, room: &str, device: &str) -> String {
        let devices = match self.house.get_room_devices(room) {
            Some(dev) => dev,
            None => return "no such room in the house".to_string(),
        };
        let dev = match devices.get(device) {
            Some(dev) => dev,
            None => return "no such device in this room in the house".to_string(),
        };
        match dev {
            Device::Socket(&ref socket) => {
                if ptr::eq(socket, &self.socket) {
                    format!("room: {}, device: {} is present", room, device)
                } else {
                    format!(
                        "room: {}, device: {} is absent in DeviceInfoProvider",
                        room, device
                    )
                }
            }
            _ => format!(
                "room: {}, device: {} is absent in DeviceInfoProvider",
                room, device
            ),
        }
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub house: &'a SmartHouse<'a>,
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn device_report(&self, room: &str, device: &str) -> String {
        let devices = match self.house.get_room_devices(room) {
            Some(dev) => dev,
            None => return "no such room in the house".to_string(),
        };
        let dev = match devices.get(device) {
            Some(dev) => dev,
            None => return "no such device in this room in the house".to_string(),
        };
        match dev {
            Device::Socket(&ref socket) => {
                if ptr::eq(socket, self.socket) {
                    format!("room: {}, device: {} is present", room, device)
                } else {
                    format!(
                        "room: {}, device: {} is absent in DeviceInfoProvider",
                        room, device
                    )
                }
            }
            Device::Thermo(&ref thrm) => {
                if ptr::eq(thrm, self.thermo) {
                    format!("room: {}, device: {} is present", room, device)
                } else {
                    format!(
                        "room: {}, device: {} is absent in DeviceInfoProvider",
                        room, device
                    )
                }
            }
        }
    }
}
// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

mod tests {
    use super::*;
    use std::collections::HashMap;

    fn setup_house<'a>(
        device0: &'a Device,
        device1: &'a Device,
        device2: &'a Device,
        device3: &'a Device,
    ) -> SmartHouse<'a> {
        let house = SmartHouse::new();
        let devices0 = HashMap::from([("device0", device0), ("device1", device1)]);
        let devices1 = HashMap::from([("device0", device2), ("device1", device3)]);
        house.set_devices(HashMap::from([("room0", devices0), ("room1", devices1)]))
    }

    #[test]
    fn test_owning_device_info_provider() {
        let socket1 = SmartSocket {};
        let mut info_provider_1 = OwningDeviceInfoProvider {
            house: SmartHouse::new(),
            socket: socket1,
        };

        let dev_sock1 = Device::Socket(&info_provider_1.socket);

        info_provider_1.house = setup_house(
            &dev_sock1,
            &Device::Thermo(&SmartThermometer {}),
            &Device::Socket(&SmartSocket {}),
            &Device::Thermo(&SmartThermometer {}),
        );
        assert_eq!(
            info_provider_1.device_report("room0", "device0"),
            "room: room0, device: device0 is present"
        );
        assert_eq!(
            info_provider_1.device_report("room0", "device1"),
            "room: room0, device: device1 is absent in DeviceInfoProvider"
        );
        assert_eq!(
            info_provider_1.device_report("room1", "device0"),
            "room: room1, device: device0 is absent in DeviceInfoProvider"
        );
        assert_eq!(
            info_provider_1.device_report("room1", "device1"),
            "room: room1, device: device1 is absent in DeviceInfoProvider"
        );
    }

    #[test]
    fn test_borrowing_device_info_provider() {
        let socket2 = SmartSocket {};
        let thermo = SmartThermometer {};

        let dev_sock2 = Device::Socket(&socket2);
        let dev_thermo = Device::Thermo(&thermo);

        let house = setup_house(
            &dev_sock2,
            &dev_thermo,
            &Device::Socket(&SmartSocket {}),
            &Device::Thermo(&SmartThermometer {}),
        );
        let info_provider_2 = BorrowingDeviceInfoProvider {
            house: &house,
            socket: &socket2,
            thermo: &thermo,
        };
        assert_eq!(
            info_provider_2.device_report("room0", "device0"),
            "room: room0, device: device0 is present"
        );
        assert_eq!(
            info_provider_2.device_report("room0", "device1"),
            "room: room0, device: device1 is present"
        );
        assert_eq!(
            info_provider_2.device_report("room1", "device0"),
            "room: room1, device: device0 is absent in DeviceInfoProvider"
        );
        assert_eq!(
            info_provider_2.device_report("room1", "device1"),
            "room: room1, device: device1 is absent in DeviceInfoProvider"
        );
    }
}
