
trait Database {
    fn get_user_name(self: &Self) -> String;
}

struct Postgres {
    user_table: String,
}

struct MongoDB {
    user_collection: String,
}

impl Database for MongoDB {
    fn get_user_name(self: &Self) -> String {
        return self.user_collection.clone();
    }
}

impl Database for Postgres {
    fn get_user_name(self: &Self) -> String {
        return self.user_table.clone();
    }
}

fn run_http_server(db: impl Database) {
    let user_name = db.get_user_name();
    println!("server started");
    println!("{user_name}");
}

fn main() {
    let p_db = Postgres {
        user_table: String::from("sathwik"),
    };

    let m_db = MongoDB {
        user_collection: String::from("gowtham"),
    };

    run_http_server(p_db);
    //run_http_server(m_db);
}
