use anyhow::{Result};
use std::{collections::HashMap, fs::OpenOptions, io::Write};

// --- 领域模型 ---

// 罪状一：用一个“大而全”的枚举来区分设备，缺乏扩展性 (违反 OCP, LSP)
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
    brightness: Option<u8>, // 只有灯泡用
    temperature: Option<f32>, // 只有恒温器用
    volume: Option<u8>,     // 只有音箱用
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
        Self { devices: HashMap::new() }
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

    // 罪状四：高层模块直接依赖底层实现细节（网络、日志） (违反 DIP)
    fn send_light_command(&self, id: &str, is_on: bool, brightness: Option<u8>) -> Result<()> {
        let state = if is_on { "on" } else { "off" };
        let brightness_str = brightness.map_or("".to_string(), |b| format!(", brightness: {}%", b));
        let log_msg = format!("[CMD] Sending to light {}: state={}, {}\n", id, state, brightness_str);

        // 模拟网络调用
        println!("{}", log_msg.trim());

        // 模拟将日志直接写入文件
        let mut log_file = OpenOptions::new().append(true).create(true).open("smart_home.log")?;
        log_file.write_all(log_msg.as_bytes())?;
        Ok(())
    }

    // ... 其他类似 set_thermostat_temp, play_speaker_music 等直接实现的方法 ...
    fn set_thermostat_temp(&self, _id: &str, _temp: f32) -> Result<()> { /* ... */ Ok(()) }
    fn play_speaker_music(&self, _id: &str, _playlist: &str) -> Result<()> { /* ... */ Ok(()) }
    fn stop_speaker_music(&self, _id: &str) -> Result<()> { /* ... */ Ok(()) }
}


// --- 使用示例 ---
fn main() -> Result<()> {
    let mut controller = SmartHomeController::new();
    controller.add_device(Device {
        id: "living_room_light".to_string(),
        device_type: DeviceType::SmartLight,
        is_on: false, brightness: Some(0), temperature: None, volume: None
    });
    controller.add_device(Device {
        id: "bedroom_thermostat".to_string(),
        device_type: DeviceType::Thermostat,
        is_on: true, temperature: Some(20.0), brightness: None, volume: None,
    });
    controller.add_device(Device {
        id: "kitchen_speaker".to_string(),
        device_type: DeviceType::SmartSpeaker,
        is_on: false, volume: Some(0), brightness: None, temperature: None,
    });

    controller.run_routine(Routine::GoodMorning)?;
    controller.run_routine(Routine::GoToBed)?;

    Ok(())
}