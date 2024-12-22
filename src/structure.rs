struct Rectangle{
width:u32,
length:u32
}

impl Rectangle{
    fn area(&self)->u32{
        self.width*self.length
    }

    fn resize(&mut self,width:u32,length:u32){
        self.width=width;
        self.length=length;
    }
}


fn main() {
 let mut r1:Rectangle=Rectangle{
     width:10,
     length:20
 };
 let res=r1.area();
 println!("{res}");

 r1.resize(10,30);

let res2=r1.area();
println!("{res2}")

}
