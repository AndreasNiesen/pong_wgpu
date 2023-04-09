use std::path::Path;
use std::fmt;
use std::fs::*;
use std::io::Write;
use chrono;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

const PATH: &'static str = "logs";

#[derive(Debug)]
pub struct Logger;

impl Logger {
    const LEVEL: LogLevel = LogLevel::High;  // TODO: Set to Low - High only for debugging purposes
    const OUTPUT: LogOutput = LogOutput::All;

    pub fn log(level: LogLevel, message: &str, typ: LogType) {
        let path = format!("{}/{}.log", PATH, chrono::offset::Local::now().format("%Y%m%d_%H"));
        
        if !Path::new(PATH).exists() {
            create_dir_all(PATH).unwrap();
        }

        let file: Option<File> = Some(OpenOptions::new().append(true).create(true).open(path).unwrap());

        if level as i32 <= Logger::LEVEL as i32 {
            let cur_time = chrono::offset::Local::now().format("%Y%m%d_%H");
            let msg = format!("{cur_time}: {} -- {message} {LINE_ENDING}", typ.to_string());

            match Logger::OUTPUT {
                LogOutput::Console => println!("{}", msg),
                LogOutput::File => {
                    file.as_ref().unwrap().write_all(msg.as_bytes()).unwrap();
                },
                LogOutput::All => {
                    println!("{}", msg);
                    file.as_ref().unwrap().write_all(msg.as_bytes()).unwrap();
                },
                LogOutput::None => ()
            }
        }
    }

    pub fn log_info(level: LogLevel, message: &str) {
        Logger::log(level, message, LogType::Info);
    }

    pub fn log_warning(level: LogLevel, message: &str) {
        Logger::log(level, message, LogType::Warning);
    }
    
    pub fn log_error(level: LogLevel, message: &str) {
        Logger::log(level, message, LogType::Error);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LogLevel {
    None,
    Low,
    Medium,
    High
}

#[derive(Debug, Copy, Clone)]
pub enum LogOutput {
    None,
    Console,
    File,
    All
}

#[derive(Debug, Copy, Clone)]
pub enum LogType {
    Info,
    Warning,
    Error
}

impl fmt::Display for LogType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}