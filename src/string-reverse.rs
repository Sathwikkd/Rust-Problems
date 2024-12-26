fn main() {
    let str1 = "hello";
    let res = rev_string(&str1);
    println!("{res}")
}

fn rev_string(st1: &str) -> String {
    st1.chars().rev().collect()
}
