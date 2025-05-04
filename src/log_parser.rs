use regex::Regex;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" => Some(LogLevel::Warn),
            "ERROR" => Some(LogLevel::Error),
            _ => None,
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            LogLevel::Debug => "\x1b[90m",  // Gray
            LogLevel::Info => "\x1b[37m",   // White
            LogLevel::Warn => "\x1b[33m",   // Yellow
            LogLevel::Error => "\x1b[31m",  // Red
        }
    }
}

pub struct LogParser {
    regex: Regex,
}

impl LogParser {
    pub fn new() -> Self {
        let regex = Regex::new(r"^\[(.*?)\] \[(DEBUG|INFO|WARN|ERROR)\] (.*)").unwrap();
        Self { regex }
    }

    pub fn parse(&self, content: &str) -> Vec<LogEntry> {
        let mut entries = Vec::new();
        let mut current_entry: Option<LogEntry> = None;

        for line in content.lines() {
            if let Some(caps) = self.regex.captures(line) {
                if let Some(entry) = current_entry.take() {
                    entries.push(entry);
                }

                if let Some(level) = LogLevel::from_str(&caps[2]) {
                    current_entry = Some(LogEntry {
                        timestamp: caps[1].to_string(),
                        level,
                        message: caps[3].to_string(),
                        lines: vec![line.to_string()],
                    });
                }
            } else if let Some(entry) = &mut current_entry {
                entry.lines.push(line.to_string());
            } else {
                // If no current entry and line doesn't match regex, create a new entry
                entries.push(LogEntry {
                    timestamp: "".to_string(),
                    level: LogLevel::Debug,
                    message: line.to_string(),
                    lines: vec![line.to_string()],
                });
            }
        }

        if let Some(entry) = current_entry {
            entries.push(entry);
        }

        entries
    }
} 