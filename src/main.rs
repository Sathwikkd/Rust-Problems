fn main(){
    let mut s1:String=String::from("hi skd");
    let res=len_finder(&mut s1);
    println!("length of the string is -> {}",res);
}

fn len_finder(s2:& mut String)->usize{
    s2.len()

}
