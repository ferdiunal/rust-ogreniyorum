use std::io::{self, Write};

trait BaseUser {
    fn send_message(&self, message: &str);
    fn get_name(&self) -> &'static str;
}

struct User {
    name: &'static str,
}

impl BaseUser for User {
    fn send_message(&self, message: &str) {
        println!("{}: {}", self.name, message);
    }

    fn get_name(&self) -> &'static str {
        self.name
    }
}

struct Chat {
    name: &'static str,
    users: Vec<Box<dyn BaseUser>>,
}

impl Chat {
    fn add(&mut self, user: Box<dyn BaseUser>) -> bool {
        let exists = self
            .users
            .iter()
            .filter(|u| u.get_name() == user.get_name())
            .count()
            > 0;

        if !exists {
            self.users.push(user);
        }

        return true;
    }
}

fn main() {
    let mut chat = Chat {
        name: "Chat",
        users: vec![],
    };

    loop {
        let mut input = String::new();
        if let Err(e) = io::stdout().flush() {
            eprintln!("Error flushing stdout: {}", e);
        }

        println!("Enter your name: ");
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading line: {}", e);
        }

        let name = Box::leak(input.trim().to_string().into_boxed_str());
        let user = User { name };

        chat.add(Box::new(user));

        println!("Welcome to the chat, {}", name);

        if chat.users.len() == 2 {
            break;
        }
    }

    chat.users.iter().for_each(|user| {
        user.send_message("User joined the chat");
    });
}
