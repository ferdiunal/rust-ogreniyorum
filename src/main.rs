#[derive(Debug, Default)]
struct User {
    name: String,
    age: u8,
    email: String,
}

#[derive(Debug, Default)]
struct UserBuilder {
    user: User,
}

impl UserBuilder {
    fn new() -> Self {
        Self {
            user: User::default(),
        }
    }

    fn name(mut self, name: String) -> Self {
        self.user.name = name;
        self
    }

    fn age(mut self, age: u8) -> Self {
        self.user.age = age;
        self
    }

    fn email(mut self, email: String) -> Self {
        self.user.email = email;
        self
    }

    fn build(self) -> User {
        self.user
    }
}

fn main() {
    let user = UserBuilder::new()
        .name("John Doe".to_string())
        .age(25)
        .email("john.doe@example.com".to_string())
        .build();

    println!("{:?}", user);
}
