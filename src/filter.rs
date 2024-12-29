fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let iter = vec.iter().filter(|x| *x % 2 == 0).map(|x| x * 2);
    let vec2: Vec<i32> = iter.collect();
    println!("{vec2:?}")
}
