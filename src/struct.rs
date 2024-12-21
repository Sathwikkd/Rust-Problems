
fn main(){
   let shape= create_shape(10,20);
    let res=area_calc(shape);
    println!("area of a given shape is ->{res}");

}
struct Shape{
    width:u32,
    length:u32
}
fn area_calc(shape:Shape)->u32{
    shape.width*shape.length
}
fn create_shape(width:u32,length:u32)->Shape{
  return  Shape{
        width:width,
        length:length
    };
}
