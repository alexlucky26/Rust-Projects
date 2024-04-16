use smart_house2::devices::device_info_providers::{
    BorrowingDeviceInfoProvider, DeviceInfoProvider, OwningDeviceInfoProvider,
};
use smart_house2::devices::smart_devices::{SmartSocket, SmartThermometer};
use smart_house2::smart_house::{SmartHouse, SmartHouseError};
use std::io;
use std::io::BufRead;

fn main() {
    // Инициализация дома
    let mut house = SmartHouse::new();

    // Инициализация дефолтных устройств
    let socket1 = SmartSocket {
        state: true,
        name: ("Kitchen".to_string(), "Smart Socket 35".to_string()),
    };
    let socket2 = SmartSocket {
        state: false,
        name: ("Bedroom".to_string(), "Smart Socket 99".to_string()),
    };
    let thermo = SmartThermometer {
        temperature: 24,
        name: ("Kitchen".to_string(), "Smart Thermometer 113".to_string()),
    };

    let mut info_provider_1 = OwningDeviceInfoProvider::new();
    info_provider_1.sockets.push(socket1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let mut sockets = Vec::new();
    sockets.push(socket2);
    let mut thermos = Vec::new();
    thermos.push(thermo);
    let mut info_provider_2 = BorrowingDeviceInfoProvider::new(&mut sockets, &mut thermos);

    loop {
        println!("Введите номер пункта:\n1. Список всех устройств и комнат.\n2. Отчет провайдеров о статусе устройств.\n3. Добавление или удаление устройств или комнат.\nЛюбое иное число. Выход.");
        let mut input_text = String::new();
        let mut stdin = io::stdin().lock();
        stdin
            .read_line(&mut input_text)
            .expect("Ошибка чтения ввода.");
        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                match i {
                    1 => {
                        println!("Вы выбрали номер: {}\nСписок всех устройств и комнат.", i);
                        println!("Текущий список комнат и устройств: {:?}", house.rooms);
                    }
                    2 => {
                        println!(
                            "Вы выбрали номер: {}\nОтчет провайдеров о статусе устройств.\n",
                            i
                        );
                        // Строим отчёт с использованием `OwningDeviceInfoProvider`.
                        let report1 = house.create_report(&info_provider_1);

                        // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
                        let report2 = house.create_report(&info_provider_2);

                        //P.S. Я не стал выводить отдельно список устройств и комнат, поскольку вывожу всё вместе со статусом в отчете
                        // Выводим отчёты на экран:
                        let result1 = match report1 {
                            Ok(r) => format!("Report #1: {r}"),
                            Err(e) => {
                                println!("{}", e);
                                format!("Error: {:#?}", e)
                            }
                        };
                        let result2 = match report2 {
                            Ok(r) => format!("Report #2: {r}"),
                            Err(e) => {
                                println!("{}", e);
                                format!("Error: {:#?}", e)
                            }
                        };

                        println!("{}", result1);
                        println!("{}", result2);
                    }
                    3 => {
                        loop {
                            println!("Вы выбрали номер: {}\nДобавление или удаление устройств или комнат.", i);
                            println!("Введите номер пункта:\n1. Добавить комнату.\n2. Добавить устройство.\n3. Удалить комнату.\n4. Удалить устройство.\n");
                            let mut add_smth = String::new();
                            stdin
                                .read_line(&mut add_smth)
                                .expect("Ошибка чтения ввода.");

                            let trimmed = add_smth.trim();
                            match trimmed.parse::<u32>() {
                                Ok(i) => match i {
                                    1 => {
                                        println!(
                                            "Текущий список комнат и устройств: {:?}",
                                            house.rooms
                                        );
                                        loop {
                                            println!("Введите название комнаты:");
                                            let mut room_name = String::new();
                                            stdin
                                                .read_line(&mut room_name)
                                                .expect("Ошибка чтения ввода.");
                                            let trimmed = room_name.trim();
                                            let result = house.add_room(trimmed.to_string());
                                            match result {
                                                    Ok(_) => { println!("Комната '{}' была добавлена.", trimmed); break; },
                                                    Err(_) => println!("Комната '{}' уже существует! Введите корректное название.", trimmed)
                                                }
                                        }
                                        break;
                                    }
                                    2 => {
                                        println!(
                                            "Текущий список комнат и устройств: {:?}",
                                            house.rooms
                                        );
                                        loop {
                                            println!("Введите название комнаты:");
                                            let mut room_name = String::new();
                                            stdin
                                                .read_line(&mut room_name)
                                                .expect("Ошибка чтения ввода.");
                                            let trimmed_room = room_name.trim();
                                            let result =
                                                house.rooms.iter().find(|r| r.0 == trimmed_room);
                                            match result {
                                                    Some(_) => {
                                                        println!("Выберите тип устройства для добавления в комнату {}.\n1. Умная розетка.\n2. Умный термометр.", trimmed_room);
                                                        loop {
                                                            let mut device_type = String::new();
                                                            stdin.read_line(&mut device_type)
                                                                .expect("Ошибка чтения ввода.");

                                                            let trimmed_type = device_type.trim();
                                                            match trimmed_type.parse::<u32>() {
                                                                Ok(i) => {
                                                                    match i {
                                                                        1 => {
                                                                            println!("Введите название розетки:");
                                                                            let mut device_name = String::new();
                                                                            stdin.read_line(&mut device_name)
                                                                                .expect("Ошибка чтения ввода.");
                                                                            let trimmed_device = device_name.trim();
                                                                            match house.add_device_to_room(trimmed_room, trimmed_device) {
                                                                                Ok(_) => {
                                                                                    let socket_to_add = SmartSocket {
                                                                                        state: true,
                                                                                        name: (trimmed_room.to_string(), trimmed_device.to_string()),
                                                                                    };
                                                                                    info_provider_1.sockets.push(socket_to_add.clone());
                                                                                    info_provider_2.sockets.push(socket_to_add);
                                                                                    println!("Устройство '{}' было добавлено в комнату {}.", trimmed_device, trimmed_room);
                                                                                    break;
                                                                                }
                                                                                Err(e) => { println!("Такое устройство уже есть в комнате! ({:?})", e); break; }
                                                                            }
                                                                        },
                                                                        2 => {
                                                                            println!("Введите название термометра:");
                                                                            let mut device_name = String::new();
                                                                            stdin.read_line(&mut device_name)
                                                                                .expect("Ошибка чтения ввода.");
                                                                            let trimmed_device = device_name.trim();
                                                                            match house.add_device_to_room(trimmed_room, trimmed_device) {
                                                                                Ok(_) => {
                                                                                    let thermo_to_add = SmartThermometer {
                                                                                        temperature: 28,
                                                                                        name: (trimmed_room.to_string(), trimmed_device.to_string()),
                                                                                    };
                                                                                    info_provider_2.thermos.push(thermo_to_add);
                                                                                    println!("Устройство '{}' было добавлено в комнату {}.", trimmed_device, trimmed_room);
                                                                                    break;
                                                                                }
                                                                                Err(e) => { println!("Такое устройство уже есть в комнате! ({:?})", e); break; }
                                                                            }
                                                                        },
                                                                        _ => {
                                                                            println!("Вы ввели неправильное значение. ({})", i);
                                                                        }
                                                                    }
                                                                },
                                                                Err(e) => {
                                                                    println!("Вы ввели неправильное значение. ({})", e);
                                                                }
                                                            }
                                                        }
                                                        break;
                                                    },
                                                    None => println!("Комната '{}' не существует! Введите корректное название.", room_name)
                                                }
                                        }
                                        break;
                                    }
                                    3 => {
                                        loop {
                                            println!("Введите название комнаты:");
                                            let mut room_name = String::new();
                                            stdin
                                                .read_line(&mut room_name)
                                                .expect("Ошибка чтения ввода.");
                                            let trimmed = room_name.trim();
                                            match house.remove_room(trimmed.to_string()) {
                                                Ok(_) => {
                                                    println!("Комната {} удалена.", trimmed);
                                                    info_provider_1
                                                        .delete_all_devices_for_room(trimmed);
                                                    info_provider_2
                                                        .delete_all_devices_for_room(trimmed);
                                                    break;
                                                }
                                                Err(_) => println!(
                                                    "Комната {} не была найдена.\nВведите ещё раз.",
                                                    trimmed
                                                ),
                                            }
                                        }
                                        break;
                                    }
                                    4 => {
                                        loop {
                                            println!("Введите название комнаты:");
                                            let mut room_name = String::new();
                                            stdin
                                                .read_line(&mut room_name)
                                                .expect("Ошибка чтения ввода.");
                                            println!("Введите название устройства:");
                                            let mut device_name = String::new();
                                            stdin
                                                .read_line(&mut device_name)
                                                .expect("Ошибка чтения ввода.");
                                            let trimmed_room = room_name.trim();
                                            let trimmed_device = device_name.trim();
                                            if info_provider_1
                                                .delete_device_from_provider(
                                                    trimmed_room,
                                                    trimmed_device,
                                                )
                                                .is_ok()
                                            {
                                                println!("Устройство {}, находящееся в комнате {} удалено из отчета Owning провайдера.\n", trimmed_device, trimmed_room);
                                            }
                                            if info_provider_2
                                                .delete_device_from_provider(
                                                    trimmed_room,
                                                    trimmed_device,
                                                )
                                                .is_ok()
                                            {
                                                println!("Устройство {}, находящееся в комнате {} удалено из отчета Borrowing провайдера.\n", trimmed_device, trimmed_room);
                                            }
                                            match house.remove_device_from_room(trimmed_room, trimmed_device) {
                                                    Ok(_) => { println!("Комната {} удалена.", trimmed_room); break; },
                                                    Err(e) => {
                                                        match e {
                                                            SmartHouseError::RoomIsNotFound => println!("Комната {} не была найдена.\nВведите ещё раз.", trimmed_room),
                                                            _ => println!("Устройство {} в комнате {} не было найдено.\nВведите ещё раз данные.", trimmed_device, trimmed_room)
                                                        }
                                                    },
                                                }
                                        }
                                        break;
                                    }
                                    _ => {
                                        println!("Вы ввели несуществующий номер пункта! ({})", i)
                                    }
                                },
                                Err(e) => {
                                    println!("Вы ввели неправильное значение. ({})", e)
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Завершаю работу.");
                        break;
                    }
                }
            }
            Err(..) => println!(
                "Ваша строка не является одним из номеров пунктов: {}",
                trimmed
            ),
        };
    }
}
