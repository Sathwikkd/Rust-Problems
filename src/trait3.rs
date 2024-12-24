trait user {
    fn validation(&self) -> String;
}
struct Client {
    name: String,
}
struct Admin {
    name: String,
}

impl user for Client {
    fn validation(&self) -> String {
        if self.name == "client" {
            return self.name.clone();
        }
        return String::from("error");
    }
}

impl user for Admin {
    fn validation(&self) -> String {
        if self.name == "admin" {
            return self.name.clone();
        }
        return String::from("error");
    }
}

fn login(log: impl user) {
    let log1 = log.validation();
    println!("login successful...");
    println!("{log1}");
}

fn main() {
    let user1 = Client {
        name: String::from("client"),
    };

    let user2 = Admin {
        name: String::from("admin"),
    };

    login(user2);
}
