use colored::Colorize;
use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, Error, ErrorKind},
    process,
};

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd)]
enum LogLevel {
    INFO = 0,
    WARN = 1,
    ERROR = 2,
}

pub struct Log {
    log_level: LogLevel,
    output_file: Option<String>,
}

impl Log {
    fn new() -> Self {
        Self {
            log_level: LogLevel::WARN,
            output_file: None,
        }
    }
    fn fmt(log_type: LogLevel, spacing: &str, message: &str) -> String {
        match log_type {
            LogLevel::INFO => format!("[{}]{} {}", "Info".bold().green(), spacing, message),
            LogLevel::WARN => format!("[{}]{} {}", "Warn".bold().yellow(), spacing, message),
            LogLevel::ERROR => format!("[{}]{} {}", "Error".bold().red(), spacing, message),
        }
    }

    pub fn info(message: &str) {
        let this = Self::new();
        if this.output_file.is_some() {
            this.info_file(message);
            return;
        }
        this.info_stdout(message);
    }
    pub fn warn(message: &str) {
        let this = Self::new();
        if this.output_file.is_some() {
            this.warn_file(message);
            return;
        }
        this.warn_stdout(message);
    }
    pub fn critical(message: &str) {
        let this = Self::new();
        if this.output_file.is_some() {
            this.critical_file(message);
        }
        this.critical_stdout(message);
    }
    pub fn critical_debug(message: &str, file: &str, line: u32) {
        let this = Self::new();
        if this.output_file.is_some() {
            this.critical_debug_file(file, line, message);
        }
        this.critical_debug_stdout(file, line, message);
    }

    pub fn info_stdout(&self, message: &str) {
        if self.log_level > LogLevel::INFO {
            return;
        }
        println!("{}", Log::fmt(LogType::INFO, "", message));
    }
    pub fn info_file(&self, message: &str) {
        if self.log_level > LogLevel::INFO {
            return;
        }
        self.to_file(&Log::fmt(LogType::INFO, "", message));
    }

    pub fn warn_stdout(&self, message: &str) {
        if self.log_level > LogLevel::WARN {
            return;
        }
        println!("{}", Log::fmt(LogType::WARN, "", message));
    }
    pub fn warn_file(&self, message: &str) {
        if self.log_level > LogLevel::WARN {
            return;
        }
        self.to_file(&Log::fmt(LogType::WARN, "", message));
    }

    pub fn critical_stdout(&self, message: &str) -> ! {
        println!("{}", Log::fmt(LogLevel::ERROR, "", message));
        process::exit(1);
    }
    pub fn critical_file(&self, message: &str) -> ! {
        self.to_file(&Log::fmt(LogLevel::ERROR, "", message));
        process::exit(1);
    }

    pub fn critical_debug_stdout(&self, file: &str, line: u32, message: &str) -> ! {
        println!(
            "{}",
            Log::fmt(
                LogLevel::ERROR,
                &format!("[{}:{}]", file, line),
                message,
            )
        );
        process::exit(1);
    }
    pub fn critical_debug_file(&self, file: &str, line: u32, message: &str) -> ! {
        self.to_file(&Log::fmt(
            LogLevel::ERROR,
            &format!("[{}:{}]", file, line),
            message,
        ));
        process::exit(1);
    }

    fn to_file(&self, data: &String) {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.output_file.clone().unwrap());
        let err = |t: &str, e: Error| {
            println!(
                "{}\n{}",
                Log::fmt(LogLevel::ERROR, "", &format!("Failed to {} the file", t)),
                e
            );
        };
        match file {
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    err("open", e);
                    return;
                }
                match File::create(self.output_file.clone().unwrap().clone()) {
                    Ok(mut f) => {
                        f.write(data.as_bytes()).unwrap_or_else(|e| {
                            err("write", e);
                            0
                        });
                    }
                    Err(e) => err("create", e),
                }
            }
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", data) {
                    err("open", e);
                }
            }
        }
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    (info $($e: expr),*) => {
        crate::log::Log::info(&format!($($e),*));
    };
    (info crate $($e: expr),*) => {
        crate::log::Log::info(&format!($($e),*));
    };
    // (info object $obj: expr) => {
    //     crate::log::Log::info(
    //         &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
    //     );
    // };
    (warn $($e: expr),*) => {
        crate::log::Log::warn(&format!($($e),*));
    };
    (warn crate $($e: expr),*) => {
        crate::log::Log::warn(&format!($($e),*));
    };
    // (warn object $obj: expr) => {
    //     crate::log::Log::warn(
    //         &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
    //     );
    // };
    (err $($e: expr),*) => {
        crate::log::Log::critical_debug(&format!($($e),*), file!(), line!());
    };
    (err crate $($e: expr),*) => {
        crate::log::Log::critical_debug(&format!($($e),*), file!(), line!());
    };
    // (err object $obj: expr) => {
    //     crate::log::Log::critical_debug(
    //         &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
    //     );
    // };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    (info $($e: expr),*) => {
        crate::log::Log::info(&format!($($e),*));
    };
    (info crate $($e: expr),*) => {
        crate::log::Log::info(&format!($($e),*));
    };
    (info object $obj: expr) => {
        crate::log::Log::info(
            &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
        );
    };
    (warn $($e: expr),*) => {
        crate::log::Log::warn(&format!($($e),*));
    };
    (warn crate $($e: expr),*) => {
        crate::log::Log::warn(&format!($($e),*));
    };
    (warn object $obj: expr) => {
        crate::log::Log::warn(
            &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
        );
    };
    (err $($e: expr),*) => {
        crate::log::Log::critical(&format!($($e),*));
    };
    (err crate $($e: expr),*) => {
        crate::log::Log::critical(&format!($($e),*));
    };
    (err object $obj: expr) => {
        crate::log::Log::critical(
            &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
        );
    };
}
