use crate::para::{SubSystem, TestCaseType};
use std::fmt;
use chrono::prelude::*;

/// 根据子系统并结合当前时间生成 vincode
#[derive(Debug, PartialEq, Eq)]
pub struct VinCode(String, DateTime<Local>);

impl VinCode {
    /// 生成 vincode
    pub fn new(subsys: SubSystem, casetype: TestCaseType) -> Self {
        // 不会转换出错
        let subsys: &str = subsys.try_into().unwrap();
        let casetype: &str = casetype.try_into().unwrap();

        // 获取时间戳，生成 vin code
        let timestamp = Local::now().timestamp_millis();
        let current_time = Local.timestamp_millis(timestamp);
        let vincode = VinCode(subsys.to_string() + casetype, current_time);

        vincode.append(&timestamp.to_string())
    }

    /// 拼接
    fn append(mut self, code: &str) -> Self {
        self.0 += code;

        self
    }

    /// 获取 vin code
    pub fn get_vincode(&self) -> &str {
        &self.0
    }

    /// 获取 timestamp
    pub fn get_timestamp(&self) -> String {
        self.1.to_string()
    }
}

impl fmt::Display for VinCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "+ vin code: {} \n+ generation time: {}", self.0, self.1)
    }
}

impl From<VinCode> for String {
    fn from(vin: VinCode) -> Self {
        format!("vin code: {}\tgeneration time: {}\n\n", vin.0, vin.1)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn new_vincode_should_work() {
    //     let vincode = VinCode::new(SubSystem::OTASUBSYS, TestCaseType::NORMAL);

    //     assert_eq!(
    //         vincode,
    //         VinCode("OTASUBSYSN".to_string())
    //     );
    // }
}
