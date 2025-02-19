#[allow(special_module_name)]
mod lib;
mod models;
mod repository;

use repository::Repository;

fn main() {
    // let user = repository::UserRepository::create();
    // println!("{:?}", user);

    let users = repository::UserRepository::get_all();
    println!("Total users: {}", users.len());
    for user in users {
        println!("{:?}", user);
    }

    // let user = repository::UserRepository::update("486db8e2-5e80-4cac-b309-4e8e791da0e9");
    // println!("{:?}", user);

    // let user = repository::UserRepository::delete("a176dc67-434e-4c28-b6c0-d69a342da388");
    // println!("{}", user);
}
