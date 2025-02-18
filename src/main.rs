#[allow(special_module_name)]
mod lib;
mod models;
mod repository;

fn main() {
    // let user = repository::user::new();
    // println!("{:?}", user);

    // let users = repository::user::list();
    // println!("{:?}", users);

    let user = repository::user::find("486db8e2-5e80-4cac-b309-4e8e791da0e9");
    println!("{}", user.name());

    println!("{:?}", user);
}
