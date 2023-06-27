
pub fn closure_example(){
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 
          where T: Fn(i32, i32) -> i32 {
            func(a, b)

    }

    let sum = |a, b| a + b ;
    let prod = |a, b| a * b ;

    println!("5+4={}", use_func(5, 4, sum)) ;
    println!("5*4={}", use_func(5, 4, prod)) ;
    

}
