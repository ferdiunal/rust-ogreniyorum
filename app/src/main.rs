use attributes::say_hello;
#[cfg(target_os = "windows")]
fn platform() {
    println!("Bu kod sadece Windows'ta çalışır!");
}

#[cfg(not(target_os = "windows"))]
fn platform() {
    println!("Bu kod Windows dışında çalışır!");
}

/**
 * Pek anlamadım ama, ileride çözeriz 😅
 */
#[say_hello]
fn main() {
    platform();
}
