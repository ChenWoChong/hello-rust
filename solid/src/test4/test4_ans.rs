use anyhow::{Result, anyhow};
use std::any::Any;
use std::collections::HashMap;

// --- 契约层 (Traits) ---
pub trait Device: Send + Sync {
    // 设备的基本能力
    fn id(&self) -> &str;
    fn as_any(&self) -> &dyn Any;

    // 更优雅的能力查询方式
    fn as_switchable(&self) -> Option<&dyn Switchable> {
        None
    }
    fn as_dimmable(&self) -> Option<&dyn Dimmable> {
        None
    }
}

// 可以开关
pub trait Switchable {
    fn turn_on(&self) -> Result<()>;
    fn turn_off(&self) -> Result<()>;
}

// 可以调节亮度
pub trait Dimmable {
    fn set_brightness(&self, b: u8) -> Result<()>;
}

// 一个场景基本的能力
pub trait Routine {
    fn name(&self) -> &'static str;
    fn execute(&self, d: &DeviceCollection) -> Result<()>;
}

// 设备的集合，通过 name -> 设备能力的抽象
type DeviceCollection = HashMap<String, Box<dyn Device>>;

// --- 实现层 (Structs) ---
// 普通灯泡
pub struct SmartLight {
    pub id: String,
}
impl Device for SmartLight {
    fn id(&self) -> &str {
        &self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_switchable(&self) -> Option<&dyn Switchable> {
        Some(self)
    }
}
impl Switchable for SmartLight {
    fn turn_on(&self) -> Result<()> {
        println!("[{}] 灯已打开", self.id);
        Ok(())
    }
    fn turn_off(&self) -> Result<()> {
        println!("[{}] 灯已关闭", self.id);
        Ok(())
    }
}

// 可调光灯泡
pub struct DimmableLight {
    pub id: String,
}
impl Device for DimmableLight {
    fn id(&self) -> &str {
        &self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
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
        println!("[{}] 可调光灯已打开", self.id);
        Ok(())
    }
    fn turn_off(&self) -> Result<()> {
        println!("[{}] 可调光灯已关闭", self.id);
        Ok(())
    }
}
impl Dimmable for DimmableLight {
    fn set_brightness(&self, b: u8) -> Result<()> {
        println!("[{}] 亮度已设为 {}%", self.id, b);
        Ok(())
    }
}

// 早安场景
pub struct GoodMorningRoutine;
impl Routine for GoodMorningRoutine {
    fn name(&self) -> &'static str {
        "早安模式"
    }
    fn execute(&self, devices: &DeviceCollection) -> Result<()> {
        println!("--- 执行场景: {} ---", self.name());
        for device in devices.values() {
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

// 电影之夜场景 (新增的扩展)
pub struct MovieNightRoutine;
impl Routine for MovieNightRoutine {
    fn name(&self) -> &'static str {
        "电影之夜"
    }
    fn execute(&self, devices: &DeviceCollection) -> Result<()> {
        println!("--- 执行场景: {} ---", self.name());
        for device in devices.values() {
            if let Some(d) = device.as_dimmable() {
                d.set_brightness(20)?; // 调暗
            } else if let Some(s) = device.as_switchable() {
                s.turn_off()?; // 普通灯则关闭
            }
        }
        Ok(())
    }
}

// --- 重构后的“中枢” ---
#[derive(Default)]
pub struct SmartHomeController {
    devices: DeviceCollection,
    routines: HashMap<&'static str, Box<dyn Routine>>,
}
impl SmartHomeController {
    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.insert(device.id().to_string(), device);
    }
    pub fn add_routine(&mut self, routine: Box<dyn Routine>) {
        self.routines.insert(routine.name(), routine);
    }
    pub fn run_routine(&self, name: &str) -> Result<()> {
        match self.routines.get(name) {
            Some(routine) => routine.execute(&self.devices),
            None => Err(anyhow!("场景 '{}' 未找到", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test4::test4_ans::SmartHomeController;
    use crate::test4::test4_ans::{
        DimmableLight, GoodMorningRoutine, MovieNightRoutine, SmartLight,
    };

    #[test]
    fn test4_should_work() -> anyhow::Result<()> {
        // --- 最终的“组装工厂” ---
        // 1. 创建所有“零件”
        let light1 = SmartLight {
            id: "客厅灯".to_string(),
        };
        let light2 = DimmableLight {
            id: "卧室灯".to_string(),
        };
        let morning_routine = GoodMorningRoutine;
        let movie_routine = MovieNightRoutine; // 新增的场景

        // 2. 创建“中枢”
        let mut controller = SmartHomeController::default();

        // 3. 注册“零件”到中枢
        controller.add_device(Box::new(light1));
        controller.add_device(Box::new(light2));
        controller.add_routine(Box::new(morning_routine));
        controller.add_routine(Box::new(movie_routine));

        // 4. 执行
        controller.run_routine("早安模式")?;
        println!();
        controller.run_routine("电影之夜")?;
        Ok(())
    }
}
