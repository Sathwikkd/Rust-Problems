trait Summary {
    fn summarizer(self: &Self);
}

fn print_summary(info: impl Summary) {
    info.summarizer();
}

struct NewsPaper {
    news: String,
}

struct Tweet {
    message: String,
}

enum AttStatus {
    Login,
    Logout,
}

impl Summary for AttStatus {
    fn summarizer(self: &Self) {
        println!("hello world");
    }
}

impl Summary for Tweet {
    fn summarizer(self: &Self) {
        println!("{}", self.message);
    }
}

impl Summary for NewsPaper {
    fn summarizer(self: &Self) {
        println!("{}", self.news);
    }
}

fn main() {
    let n = NewsPaper {
        news: String::from("deccan herald"),
    };

    let t = Tweet {
        message: String::from("hello rustaceans!!"),
    };

    let att = AttStatus::Login;

    print_summary(n);
    print_summary(t);

    print_summary(att);
}
