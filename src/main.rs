use vincode_generator::para::Cli;
use clap::Parser;

fn main() {
    // 参数解析
    // subsystem - 待测子系统 [1-OTASUBSYS, 2-MAPSENSOR, 3-SENSORAPP]
    // testcasetype - 用例类别 [N-正常系, I-异常系]
    let cli = Cli::parse();
    let subsystem = cli.subsystem;
    let testcasetype = cli.testcasetype;

    println!("subsystem: {}, testcasetype: {}", subsystem, testcasetype);
}
