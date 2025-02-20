#[derive(Clone)]
struct User {
    name: String,
}

struct Wallet {
    balance: f64,
    user: User,
}

impl User {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Wallet {
    fn new(user: User) -> Self {
        Self { balance: 0.0, user }
    }
}

struct Facade {
    user: User,
    wallet: Wallet,
}

impl Facade {
    fn new(user: User) -> Self {
        let wallet_user = user.clone();
        Self {
            user,
            wallet: Wallet::new(wallet_user),
        }
    }
}

fn main() {
    let user = User::new("John");
    let facade = Facade::new(user);
    println!("User: {}", facade.user.name);
    println!("Wallet: {}", facade.wallet.balance);
}
