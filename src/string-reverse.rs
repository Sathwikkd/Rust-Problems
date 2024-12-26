use std::io;

fn main() {
    loop {
        println!("enter the string to be reversed..");
        let mut str1: String = String::new();
        io::stdin().read_line(&mut str1).expect("failed to read...");
        let res = rev_string(&str1);
        println!("{}", res);
    }
}
fn rev_string(str: &String) -> String {
    str.chars().rev().collect()
}
