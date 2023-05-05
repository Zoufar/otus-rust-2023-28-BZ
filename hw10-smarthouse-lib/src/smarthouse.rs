use crate::device::Device;
use crate::infoprovider::DeviceInfoProvider;
use std::collections::HashMap;

pub struct SmartHouse<'a> {
    name: &'a str,
    devices: HashMap<&'a str, HashMap<&'a str, &'a Device<'a>>>,
    /* todo: данные умного дома */
}

impl<'a> SmartHouse<'a> {
    pub fn new() -> Self {
        SmartHouse {
            name: "somehouse",
            devices: HashMap::new(),
        }
        //        todo!("реализовать инициализацию дома")
    }

    pub fn get_room_devices(&self, room: &str) -> Option<&HashMap<&str, &Device>> {
        self.devices.get(room)
    }

    pub fn set_devices(
        mut self,
        devices: HashMap<&'a str, HashMap<&'a str, &'a Device<'_>>>,
    ) -> Self {
        self.devices = devices;
        self
    }

    fn get_rooms(&self) -> [&str; 2] {
        let mut rooms = [""; 2];
        let mut i = 0;
        for room in self.devices.keys() {
            rooms[i] = room;
            i += 1;
            if i > 1 {
                break;
            }
        }
        rooms
        // Размер возвращаемого массива можно выбрать самостоятельно
        //todo!("список комнат")
    }

    fn devices(&self, room: &str) -> [&str; 2] {
        let mut devices = [""; 2];
        let mut i = 0;
        for device in self.devices.get(room).unwrap().keys() {
            devices[i] = device;
            i += 1;
            if i > 1 {
                break;
            }
        }
        devices
        // Размер возвращаемого массива можно выбрать самостоятельно
        //todo!("список устройств в комнате `room`")
    }

    pub fn create_report(
        &self,
        dip: &dyn DeviceInfoProvider, /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    ) -> String {
        let mut s = format!("\n{}\n", self.name);
        for room in self.get_rooms() {
            for device in self.devices(room) {
                s = format!("{}{}\n", s, dip.device_report(room, device))
            }
        }
        s
        //   todo!("перебор комнат и устройств в них для составления отчёта")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::SmartSocket;
    use crate::device::SmartThermometer;

    struct TestInfoProvider {}

    impl DeviceInfoProvider for TestInfoProvider {
        fn device_report(&self, room: &str, device: &str) -> String {
            format!("room: {} device: {} scanned", room, device)
        }
    }

    #[test]
    fn test_new() {
        let _house = SmartHouse::new();
    }

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
    fn test_get_rooms() {
        let house = setup_house(
            &Device::Socket(&SmartSocket {}),
            &Device::Thermo(&SmartThermometer {}),
            &Device::Socket(&SmartSocket {}),
            &Device::Thermo(&SmartThermometer {}),
        );
        let rooms = vec!["room0", "room1"];
        let mut vec_rooms = Vec::from(house.get_rooms());
        vec_rooms.sort();
        assert_eq!(vec_rooms, rooms);
    }

    #[test]
    fn test_get_devices() {
        let house = setup_house(
            &Device::Socket(&SmartSocket {}),
            &Device::Thermo(&SmartThermometer {}),
            &Device::Socket(&SmartSocket {}),
            &Device::Thermo(&SmartThermometer {}),
        );

        let devs0 = vec!["device0", "device1"];
        let mut dev_room0 = Vec::from(house.devices("room0"));
        dev_room0.sort();
        assert_eq!(dev_room0, devs0);
    }

    #[test]
    fn test_create_report() {
        let house = setup_house(
            &Device::Socket(&SmartSocket {}),
            &Device::Thermo(&SmartThermometer {}),
            &Device::Socket(&SmartSocket {}),
            &Device::Thermo(&SmartThermometer {}),
        );

        let tip = TestInfoProvider {};
        let s = house.create_report(&tip);
        //println!("Report #1: {}", s);
        assert!(s.contains("room: room0 device: device0 scanned"));
        assert!(s.contains("room: room0 device: device1 scanned"));
        assert!(s.contains("room: room1 device: device0 scanned"));
        assert!(s.contains("room: room1 device: device1 scanned"));
    }
}
