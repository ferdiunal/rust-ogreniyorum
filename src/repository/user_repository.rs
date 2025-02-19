// #[allow(dead_code)]
// pub mod user {
//     use crate::lib::prompt;
//     use crate::lib::DB;
//     use crate::models::User;

//     pub fn new() -> User {
//         let mut user = User::new();

//         let firstname = prompt("Adınız: ", false);
//         let lastname = prompt("Soyadınız: ", false);
//         let email = prompt("Email: ", false);
//         let password = prompt("Şifre: ", true);

//         user.firstname(firstname)
//             .lastname(lastname)
//             .email(email)
//             .password(password);

//         save(&user);

//         user
//     }

//     fn save(user: &User) -> String {
//         let file = DB::new(user.id(), serde_json::to_string(user).unwrap());
//         file.save()
//     }

//     pub fn list() -> Vec<User> {
//         DB::list::<User>("./data")
//     }

//     pub fn find(id: &str) -> User {
//         DB::find::<User>(id)
//     }
// }

use crate::lib::prompt;
use crate::lib::DB;
use crate::models::User;
use crate::repository::Repository;
pub struct UserRepository {}

impl Repository<User> for UserRepository {
    fn get_all() -> Vec<User> {
        DB::list::<User>("./data")
    }

    fn get_by_id(id: &str) -> Option<User> {
        DB::find::<User>(id)
    }

    fn create() -> User {
        let mut user = User::new();

        let firstname = prompt("Adınız: ", false);
        let lastname = prompt("Soyadınız: ", false);
        let email = prompt("Email: ", false);
        let password = prompt("Şifre: ", true);

        user.firstname(firstname)
            .lastname(lastname)
            .email(email)
            .password(password);

        let file = DB::new(user.id(), serde_json::to_string(&user).unwrap());
        file.save();
        user
    }

    fn update(id: &str) -> Option<User> {
        let current_user = DB::find::<User>(id);

        if let Some(user) = current_user {
            let mut user = user;

            let firstname = prompt(&format!("Adınız: {}", user.firstname), false);
            let lastname = prompt(&format!("Soyadınız: {}", user.lastname), false);
            let email = prompt(&format!("Email: {}", user.email), false);
            let password = prompt("Şifre", true);

            user.firstname(firstname)
                .lastname(lastname)
                .email(email)
                .password(password);

            DB::update(id, user.clone());

            Some(user)
        } else {
            None
        }
    }

    fn delete(id: &str) -> bool {
        DB::delete(id)
    }
}
