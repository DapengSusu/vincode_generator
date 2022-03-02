use crate::para::{SubSystem, TestCaseType};

// 根据子系统并结合当前时间生成 vincode

#[derive(Debug, PartialEq, Eq)]
pub struct VinCode(String);

impl VinCode {
    /// 生成 vincode
    pub fn new(subsys: SubSystem, casetype: TestCaseType) -> Self {
        // 不会转换出错
        let subsys: &str = subsys.try_into().unwrap();
        let casetype: &str = casetype.try_into().unwrap();

        Self(subsys.to_string() + &casetype.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vincode_should_work() {
        let vincode = VinCode::new(SubSystem::OTASUBSYS, TestCaseType::NORMAL);

        assert_eq!(
            vincode,
            VinCode("OTASUBSYSN".to_string())
        );
    }
}
