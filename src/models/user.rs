use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    id: String,
    firstname: String,
    lastname: String,
    email: String,
    password: String,
}

#[allow(dead_code)]
impl User {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        Self {
            id: id.to_string(),
            firstname: String::new(),
            lastname: String::new(),
            email: String::new(),
            password: String::new(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn firstname(&mut self, firstname: String) -> &mut Self {
        self.firstname = firstname;
        self
    }

    pub fn lastname(&mut self, lastname: String) -> &mut Self {
        self.lastname = lastname;
        self
    }

    pub fn email(&mut self, email: String) -> &mut Self {
        self.email = email;
        self
    }

    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }

    pub fn name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}
