use thiserror::Error;

/// 常用错误
#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Unknown subsystem: `{0}`")]
    UnknownSubSystem(String),
    #[error("Unknown testcase type: `{0}`")]
    UnknownCaseType(String),
    #[error("Parse string `{0}` to an integer failed")]
    StringToIntError(String),
    #[error("unknown error")]
    Unknown,
}
