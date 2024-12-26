fn main() {
    let mut arr = [4, 5, 3, 7, 2, 9, 0, 9];
    arr.sort();
    println!("{arr:?}");
    let last = arr.len() - 1;
    println!("largest number in an array is -> {}", arr[last])
}
