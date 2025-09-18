use anyhow::{Result, anyhow};
use std::{collections::HashMap, fs::OpenOptions, io::Write};

// --- 领域模型 ---

// 罪状一：用一个“大而全”的枚举来区分设备，缺乏扩展性 (违反 OCP, LSP)
pub mod contracts {

    pub trait Runnable {
        fn open(&mut self) -> anyhow::Result<()>;
        fn close(&mut self) -> anyhow::Result<()>;
    }

    pub trait Dimmable: Runnable {
        fn set_light(&mut self, is_on: bool, brightness: Option<u8>) -> anyhow::Result<()>;
    }

    // 罪状四：高层模块直接依赖底层实现细节（网络、日志） (违反 DIP)

    // ... 其他类似 set_thermostat_temp, play_speaker_music 等直接实现的方法 ...
    pub trait TemperatureAdjustable: Runnable {
        fn set_temperature(&mut self, _temp: f32) -> anyhow::Result<()>;
    }

    pub trait Speaker: Runnable {
        fn play_speaker_music(&mut self, _id: &str, _playlist: &str) -> anyhow::Result<()>;
        fn stop_speaker_music(&mut self, _id: &str) -> anyhow::Result<()>;
    }
    
    pub trait Routiner {
        fn run(&self) -> anyhow::Result<()>;
    }
}

pub mod devices {
    use crate::test4::contracts::{Dimmable, Runnable, Speaker, TemperatureAdjustable};
    use std::fs::OpenOptions;
    use std::io::Write;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum DeviceType {
        SmartLight,
        Thermostat,
        SmartSpeaker,
    }

    // 一个设备的所有可能状态都塞在一个结构体里 (违反 ISP)
    #[derive(Debug, Clone)]
    pub struct Device {
        pub id: String,
        pub device_type: DeviceType,
        // --- 状态字段 ---
        is_on: bool,
    }

    pub struct DeviceSmartLight {
        pub device: Device,
        pub brightness: Option<u8>, // 只有灯泡用
    }

    impl Runnable for DeviceSmartLight {
        fn open(&mut self) -> anyhow::Result<()> {
            todo!()
        }
        fn close(&mut self) -> anyhow::Result<()> {
            todo!()
        }
    }

    impl Dimmable for DeviceSmartLight {
        fn set_light(&mut self, is_on: bool, brightness: Option<u8>) -> anyhow::Result<()> {
            let state = if is_on { "on" } else { "off" };
            let brightness_str =
                brightness.map_or("".to_string(), |b| format!(", brightness: {}%", b));
            let log_msg = format!(
                "[CMD] Sending to light {}: state={}, {}\n",
                self.device.id, state, brightness_str
            );

            // 模拟网络调用
            println!("{}", log_msg.trim());

            // 模拟将日志直接写入文件
            let mut log_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("smart_home.log")?;
            log_file.write_all(log_msg.as_bytes())?;
            Ok(())
        }
    }

    pub struct DeviceThermostat {
        pub device: Device,
        pub temperature: Option<f32>, // 只有恒温器用
    }

    impl Runnable for DeviceThermostat {
        fn open(&mut self) -> anyhow::Result<()> {
            todo!()
        }
        fn close(&mut self) -> anyhow::Result<()> {
            todo!()
        }
    }

    impl TemperatureAdjustable for DeviceThermostat {
        fn set_temperature(&mut self, _temp: f32) -> anyhow::Result<()> {
            /* ... */
            Ok(())
        }
    }

    pub struct DeviceSmartSpeaker {
        pub device: Device,
        pub volume: Option<u8>, // 只有音箱用
    }

    impl Runnable for DeviceSmartSpeaker {
        fn open(&mut self) -> anyhow::Result<()> {
            todo!()
        }
        fn close(&mut self) -> anyhow::Result<()> {
            todo!()
        }
    }

    impl Speaker for DeviceSmartSpeaker {
        fn play_speaker_music(&mut self, _id: &str, _playlist: &str) -> anyhow::Result<()> {
            /* ... */
            Ok(())
        }
        fn stop_speaker_music(&mut self, _id: &str) -> anyhow::Result<()> {
            /* ... */
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Routine {
    GoodMorning,
    GoToBed,
}

// --- 违反 SOLID 的“上帝”对象 ---
pub struct SmartHomeController {
    devices: HashMap<String, Device>,
}

impl SmartHomeController {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(device.id.clone(), device);
    }

    /// 罪状二：一个方法包揽所有场景逻辑，每增新场景、新设备都需修改 (违反 SRP, OCP)
    pub fn run_routine(&self, routine: Routine) -> Result<()> {
        println!("\n--- 执行场景: {:?} ---", routine);
        match routine {
            Routine::GoodMorning => {
                for device in self.devices.values() {
                    // 罪状三：根据具体类型执行焊死的逻辑
                    match device.device_type {
                        DeviceType::SmartLight => {
                            // 细节一：直接调用具体实现
                            self.send_light_command(&device.id, true, Some(80))?;
                        }
                        DeviceType::Thermostat => {
                            self.set_thermostat_temp(&device.id, 22.5)?;
                        }
                        DeviceType::SmartSpeaker => {
                            self.play_speaker_music(&device.id, "Morning Jazz Playlist")?;
                        }
                    }
                }
            }
            Routine::GoToBed => {
                for device in self.devices.values() {
                    match device.device_type {
                        DeviceType::SmartLight => {
                            self.send_light_command(&device.id, false, None)?;
                        }
                        DeviceType::Thermostat => {
                            self.set_thermostat_temp(&device.id, 18.0)?;
                        }
                        DeviceType::SmartSpeaker => {
                            self.stop_speaker_music(&device.id)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

// --- 使用示例 ---
fn main() -> Result<()> {
    let mut controller = SmartHomeController::new();
    controller.add_device(Device {
        id: "living_room_light".to_string(),
        device_type: DeviceType::SmartLight,
        is_on: false,
        brightness: Some(0),
        temperature: None,
        volume: None,
    });
    controller.add_device(Device {
        id: "bedroom_thermostat".to_string(),
        device_type: DeviceType::Thermostat,
        is_on: true,
        temperature: Some(20.0),
        brightness: None,
        volume: None,
    });
    controller.add_device(Device {
        id: "kitchen_speaker".to_string(),
        device_type: DeviceType::SmartSpeaker,
        is_on: false,
        volume: Some(0),
        brightness: None,
        temperature: None,
    });

    controller.run_routine(Routine::GoodMorning)?;
    controller.run_routine(Routine::GoToBed)?;

    Ok(())
}
