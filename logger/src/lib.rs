/**
 * Rust Singleton Pattern Example
 * Stackoverflow: https://stackoverflow.com/a/27826181
 */
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Log {
    pub level: String,
    pub message: String,
    pub timestamp: String,
}

impl Log {
    fn new(level: String, message: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        Self {
            level,
            message,
            timestamp,
        }
    }
}

#[derive(Debug)]
pub struct Logger {
    logs: Vec<Log>,
}

impl Logger {
    fn new() -> Self {
        println!("Logger created");
        Self { logs: vec![] }
    }

    pub fn make() -> Arc<Mutex<Logger>> {
        static mut INSTANCE: Option<Arc<Mutex<Logger>>> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        unsafe {
            INIT.call_once(|| {
                INSTANCE = Some(Arc::new(Mutex::new(Logger::new())));
            });

            INSTANCE.clone().unwrap()
        }
    }

    pub fn log(level: String, message: String) {
        let logger = Logger::make();
        let mut lock = logger.lock().unwrap();
        let log = Log::new(level, message);
        lock.logs.push(log);
    }

    pub fn logs() -> Vec<Log> {
        let logger = Logger::make();
        let lock = logger.lock().unwrap();
        lock.logs.clone()
    }

    pub fn clear() {
        let logger = Logger::make();
        let mut lock = logger.lock().unwrap();
        lock.logs.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn setup() {
        Logger::clear();
    }

    #[test]
    fn test_singleton_instance() {
        let logger1 = Logger::make();
        let logger2 = Logger::make();

        assert!(
            Arc::ptr_eq(&logger1, &logger2),
            "Singleton instance'ları aynı olmalı"
        );
    }

    #[test]
    fn test_logging_functionality() {
        setup();

        Logger::log("INFO".to_string(), "Test message".to_string());

        let logs = Logger::logs();
        assert_eq!(logs.len(), 1, "Bir log kaydı olmalı");
        assert_eq!(logs[0].level, "INFO");
        assert_eq!(logs[0].message, "Test message");
    }

    #[test]
    fn test_thread_safety() {
        setup();

        let mut handles = vec![];

        for i in 0..3 {
            let handle = thread::spawn(move || {
                Logger::log("DEBUG".to_string(), format!("Thread {} message", i));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let logs = Logger::logs();
        assert_eq!(logs.len(), 3, "3 log kaydı olmalı");
    }
}
