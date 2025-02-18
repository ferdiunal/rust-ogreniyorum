use rpassword::read_password;
use std::io::{self, Write};

pub struct Prompt {
    input: String,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.input = String::new();
    }

    pub fn prompt(&mut self, label: &str) -> &mut Self {
        self.clear();
        io::stdout().flush().unwrap();

        println!("{}", label);
        io::stdin().read_line(&mut self.input).unwrap();
        self
    }

    pub fn prompt_password(&mut self, label: &str) -> &mut Self {
        self.clear();
        io::stdout().flush().unwrap();

        println!("{}", label);
        self.input = read_password().unwrap();
        self
    }
}

pub fn prompt(label: &str, is_password: bool) -> String {
    let mut prompt = Prompt::new();
    if is_password {
        prompt.prompt_password(label);
    } else {
        prompt.prompt(label);
    }

    prompt.input.trim().to_string()
}
