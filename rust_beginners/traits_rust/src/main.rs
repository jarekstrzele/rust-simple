
trait Perimeter {
    fn calc_perimeter(&self) -> i32;
}

struct Square{
    a: i32,
}

impl Perimeter for Square {
    fn calc_perimeter(&self)->i32 {
        4*self.a
    } 
}

struct Triangle{
    a: i32,
    b: i32,
    c: i32,
}

impl Perimeter for Triangle {
    fn calc_perimeter(&self) -> i32 {
        self.a + self.b + self.c
    }
}

fn perimeter(shape: impl Perimeter){
    let perimeter = shape.calc_perimeter() ;
    println!("{}", perimeter) ;

}


fn main() {
    perimeter(Square{ a:10}) ;
    perimeter(Triangle{ a:10, b:20, c:30}) ;
}
