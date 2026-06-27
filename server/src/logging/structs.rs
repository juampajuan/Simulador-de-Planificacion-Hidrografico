
/// Posibles tipos de LOG
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogType {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogType {
    pub fn level(&self) -> i32 {
        match self {
            LogType::Debug => 0,
            LogType::Info  => 1,
            LogType::Warn  => 2,
            LogType::Error => 3,
        }
    }

    pub fn toString(&self) -> &str {
        match self {
            LogType::Debug => "DEBUG",
            LogType::Info  => "INFO",
            LogType::Warn  => "WARN",
            LogType::Error => "ERROR",
        }
    }
}

/// # Structura usada para comunicar threads
///
/// Posee el string para loggear y su severidad.
#[derive(Debug)]
pub struct ThreadMessage {
    pub log_type: LogType,
    pub msg: String
}
