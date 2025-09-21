use anyhow::{Result, anyhow};
use std::{collections::HashMap, fs::OpenOptions, io::Write};

// --- 领域模型 ---
pub trait Device {
    fn get_id(&self) -> &str;

    fn as_switchable(&self) -> Option<&dyn Switchable> {
        None
    }

    fn as_dimmable(&self) -> Option<&dyn Dimmable> {
        None
    }
}

pub trait Switchable {
    fn turn_on(&self) -> Result<()>;
    fn turn_off(&self) -> Result<()>;
}

pub trait Dimmable {
    fn set_brightness(&self, bright: u8) -> Result<()>;
}

type DeviceCollection = HashMap<String, Box<dyn Device>>;

pub trait Routine {
    fn name(&self) -> &'static str;
    fn execute(&self, dc: &DeviceCollection) -> Result<()>;
}

pub struct SmartLight {
    id: String,
}

impl Device for SmartLight {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn as_switchable(&self) -> Option<&dyn Switchable> {
        Some(self)
    }
}

impl Switchable for SmartLight {
    fn turn_on(&self) -> Result<()> {
        println!("SmartLight {} turn_on", self.id);
        Ok(())
    }

    fn turn_off(&self) -> Result<()> {
        println!("SmartLight {} turn_off", self.id);
        Ok(())
    }
}

pub struct DimmableLight {
    pub id: String,
}

impl Device for DimmableLight {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn as_switchable(&self) -> Option<&dyn Switchable> {
        Some(self)
    }

    fn as_dimmable(&self) -> Option<&dyn Dimmable> {
        Some(self)
    }
}

impl Switchable for DimmableLight {
    fn turn_on(&self) -> Result<()> {
        println!("DimmableLight {} turn_on", self.id);
        Ok(())
    }

    fn turn_off(&self) -> Result<()> {
        println!("DimmableLight {} turn_off", self.id);
        Ok(())
    }
}

impl Dimmable for DimmableLight {
    fn set_brightness(&self, bright: u8) -> Result<()> {
        println!("DimmableLight {} set_brightness to {}", self.id, bright);
        Ok(())
    }
}

pub struct GoodMorningRoute;
impl Routine for GoodMorningRoute {
    fn name(&self) -> &'static str {
        "GoodMorningRoute"
    }

    fn execute(&self, dc: &DeviceCollection) -> Result<()> {
        println!("execute routine {}", self.name());
        for device in dc.values() {
            if let Some(s) = device.as_switchable() {
                s.turn_on()?;
            }
            if let Some(d) = device.as_dimmable() {
                d.set_brightness(80)?;
            }
        }
        Ok(())
    }
}

pub struct MovieNightRoutine;
impl Routine for MovieNightRoutine {
    fn name(&self) -> &'static str {
        "MovieNightRoutine"
    }
    fn execute(&self, dc: &DeviceCollection) -> Result<()> {
        println!("execute routine {}", self.name());
        for device in dc.values() {
            if let Some(d) = device.as_dimmable() {
                d.set_brightness(10)?;
            } else if let Some(s) = device.as_switchable() {
                s.turn_off()?;
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct SmartHomeController {
    devices: DeviceCollection,
    routines: HashMap<&'static str, Box<dyn Routine>>,
}

impl SmartHomeController {
    pub fn add_devices(&mut self, device: Box<dyn Device>) -> &mut Self {
        self.devices.insert(device.get_id().to_string(), device);
        self
    }

    pub fn add_routines(&mut self, routine: Box<dyn Routine>) -> &mut Self {
        self.routines.insert(routine.name(), routine);
        self
    }

    pub fn run_routine(&self, name: &str) -> Result<()> {
        match self.routines.get(name) {
            Some(r) => r.execute(&self.devices),
            None => Err(anyhow!("routine {} not found", name)),
        }
    }
}

// 罪状一：用一个“大而全”的枚举来区分设备，缺乏扩展性 (违反 OCP, LSP)
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum DeviceType {
//     SmartLight,
//     Thermostat,
//     SmartSpeaker,
// }

// 一个设备的所有可能状态都塞在一个结构体里 (违反 ISP)
// #[derive(Debug, Clone)]
// pub struct Device {
//     pub id: String,
//     pub device_type: DeviceType,
//     // --- 状态字段 ---
//     is_on: bool,
//     brightness: Option<u8>, // 只有灯泡用
//     temperature: Option<f32>, // 只有恒温器用
//     volume: Option<u8>,     // 只有音箱用
// }

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub enum Routine {
//     GoodMorning,
//     GoToBed,
// }

// --- 违反 SOLID 的“上帝”对象 ---
// pub struct SmartHomeController {
//     devices: HashMap<String, Device>,
// }

// impl SmartHomeController {
//     pub fn new() -> Self {
//         Self {
//             devices: HashMap::new(),
//         }
//     }
//
//     pub fn add_device(&mut self, device: Device) {
//         self.devices.insert(device.id.clone(), device);
//     }
//
//     /// 罪状二：一个方法包揽所有场景逻辑，每增新场景、新设备都需修改 (违反 SRP, OCP)
//     pub fn run_routine(&self, routine: Routine) -> Result<()> {
//         println!("\n--- 执行场景: {:?} ---", routine);
//         match routine {
//             Routine::GoodMorning => {
//                 for device in self.devices.values() {
//                     // 罪状三：根据具体类型执行焊死的逻辑
//                     match device.device_type {
//                         DeviceType::SmartLight => {
//                             // 细节一：直接调用具体实现
//                             self.send_light_command(&device.id, true, Some(80))?;
//                         }
//                         DeviceType::Thermostat => {
//                             self.set_thermostat_temp(&device.id, 22.5)?;
//                         }
//                         DeviceType::SmartSpeaker => {
//                             self.play_speaker_music(&device.id, "Morning Jazz Playlist")?;
//                         }
//                     }
//                 }
//             }
//             Routine::GoToBed => {
//                 for device in self.devices.values() {
//                     match device.device_type {
//                         DeviceType::SmartLight => {
//                             self.send_light_command(&device.id, false, None)?;
//                         }
//                         DeviceType::Thermostat => {
//                             self.set_thermostat_temp(&device.id, 18.0)?;
//                         }
//                         DeviceType::SmartSpeaker => {
//                             self.stop_speaker_music(&device.id)?;
//                         }
//                     }
//                 }
//             }
//         }
//         Ok(())
//     }
//
//     // 罪状四：高层模块直接依赖底层实现细节（网络、日志） (违反 DIP)
//     fn send_light_command(&self, id: &str, is_on: bool, brightness: Option<u8>) -> Result<()> {
//         let state = if is_on { "on" } else { "off" };
//         let brightness_str = brightness.map_or("".to_string(), |b| format!(", brightness: {}%", b));
//         let log_msg = format!(
//             "[CMD] Sending to light {}: state={}, {}\n",
//             id, state, brightness_str
//         );
//
//         // 模拟网络调用
//         println!("{}", log_msg.trim());
//
//         // 模拟将日志直接写入文件
//         let mut log_file = OpenOptions::new()
//             .append(true)
//             .create(true)
//             .open("smart_home.log")?;
//         log_file.write_all(log_msg.as_bytes())?;
//         Ok(())
//     }
//
//     // ... 其他类似 set_thermostat_temp, play_speaker_music 等直接实现的方法 ...
//     fn set_thermostat_temp(&self, _id: &str, _temp: f32) -> Result<()> {
//         /* ... */
//         Ok(())
//     }
//     fn play_speaker_music(&self, _id: &str, _playlist: &str) -> Result<()> {
//         /* ... */
//         Ok(())
//     }
//     fn stop_speaker_music(&self, _id: &str) -> Result<()> {
//         /* ... */
//         Ok(())
//     }
// }

// --- 使用示例 ---
// fn main() -> Result<()> {
//     let mut controller = SmartHomeController::new();
//     controller.add_device(Device {
//         id: "living_room_light".to_string(),
//         device_type: DeviceType::SmartLight,
//         is_on: false,
//         brightness: Some(0),
//         temperature: None,
//         volume: None,
//     });
//     controller.add_device(Device {
//         id: "bedroom_thermostat".to_string(),
//         device_type: DeviceType::Thermostat,
//         is_on: true,
//         temperature: Some(20.0),
//         brightness: None,
//         volume: None,
//     });
//     controller.add_device(Device {
//         id: "kitchen_speaker".to_string(),
//         device_type: DeviceType::SmartSpeaker,
//         is_on: false,
//         volume: Some(0),
//         brightness: None,
//         temperature: None,
//     });
//
//     controller.run_routine(Routine::GoodMorning)?;
//     controller.run_routine(Routine::GoToBed)?;
//
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use crate::test4::test4::{
        DimmableLight, GoodMorningRoute, MovieNightRoutine, SmartHomeController, SmartLight,
    };
    use anyhow::Result;

    #[test]
    fn smart_controller_should_work() -> Result<()> {
        let light1 = SmartLight {
            id: "客厅灯".to_string(),
        };
        let light2 = DimmableLight {
            id: "卧室灯".to_string(),
        };

        let morning_routine = GoodMorningRoute;
        let move_night_routine = MovieNightRoutine;

        let mut controller = &mut SmartHomeController::default();
        controller = controller
            .add_devices(Box::new(light1))
            .add_devices(Box::new(light2))
            .add_routines(Box::new(morning_routine))
            .add_routines(Box::new(move_night_routine));

        controller.run_routine("MovieNightRoutine")?;
        println!();
        controller.run_routine("GoodMorningRoute")?;

        Ok(())
    }
}
