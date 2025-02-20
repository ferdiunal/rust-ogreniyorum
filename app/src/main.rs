use macros;

fn main() {
    macros::say_hello!("John");
    println!("Timestamp: {}", macros::now!()); // örn: 2024-03-17 15:45:30
}
