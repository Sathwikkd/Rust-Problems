fn main() {
    let s=String::from("hi rust");
    hello(&s);
   let res= hello1(&s);
   println!("{res}")
}

fn hello(s:&String){
    println!("{s}")
}
fn hello1(s:&str)->&str{
    s
}
