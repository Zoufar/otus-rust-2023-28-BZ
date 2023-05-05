use hw10_smarthouse_lib::device::SmartThermometer;
use hw10_smarthouse_lib::device::{Device, SmartSocket};
use hw10_smarthouse_lib::infoprovider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
use hw10_smarthouse_lib::smarthouse::SmartHouse;
use std::collections::HashMap;

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
    info_provider_1.house = info_provider_1
        .house
        .set_devices(HashMap::from([("room0", devices0), ("room1", devices1)]));

    let house_ = &info_provider_1.house;

    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house_.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.

    let info_provider_2 = BorrowingDeviceInfoProvider {
        house: house_,
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house_.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
