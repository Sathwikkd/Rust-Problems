use std::io;

fn main() {
    let mut arr = [0; 5];
    println!("enter 5 elementss to array..");
    for i in 0..arr.len() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(
            "failed to read..
            ",
        );
        arr[i] = input.trim().parse().expect("failed to parse");
    }
    println!("{arr:?}")
}
