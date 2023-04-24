// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым
#![allow(clippy::needless_borrowed_reference)]

use std::collections::HashMap;
use std::ptr;

struct SmartHouse<'a> {
    devices: HashMap<&'a str, HashMap<&'a str, &'a Device<'a>>>,
    //   devices: [(&'a str, ((&'a str, Option(&'a SmartSocket)), (&'a str, &'a SmartThermometer)));2]

    /* todo: данные умного дома */
}

enum Device<'a> {
    Socket(&'a SmartSocket),
    Thermo(&'a SmartThermometer),
}

impl SmartHouse<'_> {
    fn new() -> Self {
        SmartHouse {
            devices: HashMap::new(),
        }
        //        todo!("реализовать инициализацию дома")
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

    fn create_report(
        &self,
        dip: &dyn DeviceInfoProvider, /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    ) -> String {
        let mut s = "\n".to_string();
        for room in self.get_rooms() {
            for device in self.devices(room) {
                s = format!("{} + {} + \n ", s, dip.device_report(room, device))
            }
        }
        s
        //   todo!("перебор комнат и устройств в них для составления отчёта")
    }
}

trait DeviceInfoProvider {
    fn device_report(&self, room: &str, device: &str) -> String;
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket {}

struct SmartThermometer {}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider<'a> {
    house: SmartHouse<'a>,
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider<'_> {
    fn device_report(&self, room: &str, device: &str) -> String {
        let devices = match self.house.devices.get(room) {
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

struct BorrowingDeviceInfoProvider<'a, 'b> {
    house: &'a SmartHouse<'a>,
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn device_report(&self, room: &str, device: &str) -> String {
        let devices = match self.house.devices.get(room) {
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

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let mut info_provider_1 = OwningDeviceInfoProvider {
        house,
        socket: socket1,
    };

    let socket1 = &info_provider_1.socket;

    let device0 = &Device::Socket(socket1);
    let device1 = &Device::Thermo(&thermo);
    let devices0 = HashMap::from([("device0", device0), ("device1", device1)]);
    let device2 = &Device::Socket(&socket2);
    let device3 = &Device::Thermo(&thermo);
    let devices1 = HashMap::from([("device0", device2), ("device1", device3)]);
    info_provider_1.house.devices = HashMap::from([("room0", devices0), ("room1", devices1)]);

    let house = &info_provider_1.house;

    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        house,
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
