fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let iter = vec.into_iter();
    for val in iter {
        println!("{val}")
    }
}
