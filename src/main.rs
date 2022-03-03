use vincode_generator::{para::Cli, vincode::VinCode};
use clap::Parser;
use std::{ io::{self, Write}, fs };

/// vin code 保存位置
const VINCODE_SAVE_PATH: &str = "./vincode.save";

fn main() -> io::Result<()> {
    // 参数解析
    // subsystem - 待测子系统 [1-OTASUBSYS, 2-MAPSENSOR, 3-SENSORAPP]
    // testcasetype - 用例类别 [N-正常系, I-异常系]
    let cli = Cli::parse();
    let subsystem = cli.subsystem;
    let testcasetype = cli.testcasetype;

    // 生成 vin code
    let vincode = VinCode::new(subsystem, testcasetype);

    // 显示 vin code
    println!("{}", vincode);

    // 保存 vin code
    let mut file = fs::OpenOptions::new().append(true).open(VINCODE_SAVE_PATH)?;
    let vin: String = vincode.into();

    file.write(vin.as_bytes())?;

    Ok(())
}
