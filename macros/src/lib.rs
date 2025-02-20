use std::time::{SystemTime, UNIX_EPOCH};

#[macro_export]
macro_rules! say_hello {
    ($name:expr) => {
        println!("Hello, {}", $name);
    };
}

#[macro_export]
macro_rules! now {
    () => {{
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string()
    }};
}
