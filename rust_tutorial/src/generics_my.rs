pub fn generics_examples(){
 

    let rec1: Rectangle1<i32, f64> = Rectangle1{length: 4, height: 10.5};
    println!("{:?}", rec1) ;

    let rec: Rectangle = Shape::new(10.50, 10.0) ;
    println!("area {}", rec.area()) ;
    
}
#[derive(Debug)]
struct Rectangle1<T, U>{
        length: T,
        height: U,
    } 
// --------------------
// ------------------
struct Rectangle{
    length: f32,
    width: f32,
} 

impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Rectangle {
        Rectangle {length, width}
    }

    fn area(&self)-> f32{
        self.length * self.width
    }
}

trait Shape {
    fn new(length: f32, width: f32) -> Self ;
    fn area(&self) -> f32 ;

}
   
use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T{
    x + y
}

