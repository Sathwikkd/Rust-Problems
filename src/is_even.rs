fn main() {
    let num: i32 = 13;
    let res = is_even(num);
    println!("{res}");
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
