#[allow(dead_code)]
pub mod user {
    use crate::lib::prompt;
    use crate::lib::DB;
    use crate::models::User;

    pub fn new() -> User {
        let mut user = User::new();

        let firstname = prompt("Adınız: ", false);
        let lastname = prompt("Soyadınız: ", false);
        let email = prompt("Email: ", false);
        let password = prompt("Şifre: ", true);

        user.firstname(firstname)
            .lastname(lastname)
            .email(email)
            .password(password);

        save(&user);

        user
    }

    fn save(user: &User) -> String {
        let file = DB::new(user.id(), serde_json::to_string(user).unwrap());
        file.save()
    }

    pub fn list() -> Vec<User> {
        DB::list::<User>("./data")
    }

    pub fn find(id: &str) -> User {
        DB::find::<User>(id)
    }
}
