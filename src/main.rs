pub trait Eat {
    fn eatdinner(&self) {
        println!("i eat my dish")
    }
}

pub struct Cat {
    dinner: String,
}

pub struct Dog {
    has_dinner: bool,
}

impl Eat for Cat {
    fn eatdinner(&self) {
        // println!("cat eats rat!");
    }
}
impl Eat for Dog {
    // fn eatdinner(&self) {
    //     // println!("dog had dinenr..")
    // }
}

fn main() {
    let eat1 = Cat {
        dinner: String::from("had dinenr.."),
    };

    let eat2 = Dog { has_dinner: true };
    // eat1.eatdinner();
    eat2.eatdinner();
}
