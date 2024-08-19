// (C) Psilocyborg 2024
// HENRII Car Diagnostic Tool
// File Description: Module for a simple logging system, primarily for trace/debug reasons.

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub struct Logger 
{
    level: LogLevel,
}

impl Logger
{
    pub fn new (level: LogLevel) -> Self {
        Logger {level}
    }

    pub fn log(&self, level: LogLevel, message: &str)
    {
        if self.should_log(&level)
        {
            println!("[{:?}] {}", level, message);
        }    
    }

    pub fn should_log(&self, level: &LogLevel) -> bool
    {
        match(&self.level, level)
        {
            (LogLevel::Info, _) => true,
            (LogLevel::Warning, LogLevel::Warning) => true,
            (LogLevel::Warning, LogLevel::Error) => true,
            (LogLevel::Error, LogLevel::Error) => true,
            _ => false,
        }
    }
}