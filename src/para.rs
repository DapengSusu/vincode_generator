use std::str::FromStr;
use clap::Parser;
use crate::error::{self, GeneratorError};
use std::fmt;

/// Generate vin code according to the current time.
#[derive(Parser, Debug)]
#[clap(name = "vincode_generator")]
#[clap(author = "Jumper <911079536@qq.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "Generate vin code according to the current time.", long_about = None)]
pub struct Cli {
    /// 待测子系统 - [1-OTASUBSYS, 2-MAPSENSOR, 3-SENSORAPP]
    #[clap(short, long)]
    pub subsystem: SubSystem,

    /// 用例类别 - [N-正常系, I-异常系]
    #[clap(short, long)]
    pub testcasetype: TestCaseType,
}

#[derive(Parser, Debug)]
pub enum SubSystem {
    OTASUBSYS, // 1
    MAPSENSOR, // 2
    SENSORAPP, // 3
}

impl FromStr for SubSystem {
    type Err = error::GeneratorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let subsystem = s.parse::<i32>()
            .or(Err(GeneratorError::StringToIntError(s.to_string())));
        if let Ok(subsystem) = subsystem {
            match subsystem {
                1 => Ok(SubSystem::OTASUBSYS),
                2 => Ok(SubSystem::MAPSENSOR),
                3 => Ok(SubSystem::SENSORAPP),
                _ => Err(GeneratorError::UnknownSubSystem(s.to_string())),
            }
        } else {
            Err(GeneratorError::StringToIntError(s.to_string()))
        }
    }
}

impl fmt::Display for SubSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser, Debug)]
pub enum TestCaseType {
    NORMAL,    // N
    ABNORMAL,  // I
}

impl FromStr for TestCaseType {
    type Err = error::GeneratorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "N" {
            // 正常系
            Ok(TestCaseType::NORMAL)
        } else if s == "I" {
            // 异常系
            Ok(TestCaseType::ABNORMAL)
        } else {
            // 错误类型
            Err(GeneratorError::UnknownCaseType(s.to_string()))
        }
    }
}

impl fmt::Display for TestCaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
