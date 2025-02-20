use logger::Logger;
fn main() {
    Logger::log("INFO".to_string(), "Test message".to_string());

    let logs = Logger::logs();
    println!("{:?}", logs);

    Logger::log("INFO".to_string(), "Test message 2".to_string());

    let logs = Logger::logs();
    println!("{:?}", logs);
}
