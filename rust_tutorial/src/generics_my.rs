pub fn generics_examples(){
    println!("2 + 4 = {}",get_sum_gen(2, 4)) ;
    println!("2.0 + 4.0 = {}",get_sum_gen(2, 4)) ;

}


use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T{
    x + y
}

