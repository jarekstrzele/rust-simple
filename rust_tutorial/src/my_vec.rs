
pub fn vec_example() {
    let vec: Vec<i32> = Vec::new() ;

    let mut vec2: Vec<i32> = vec![10,20,30,40] ;
    vec2.push(5) ;
    let first = vec2[0] ;
    println!("1st: {}", first) ;
    let second: &i32 = &vec2[1] ; // it reads a value 
    println!("second: {}", second);
    let sec2:i32 = vec2[1] ; // it copies a value
    println!("second: {}", sec2);

    match vec2.get(1){ //get() returns Option type
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2 ;
       
    }
    println!("------------") ;
    for i in &vec2{
        println!("{}", i) ;
    }
    println!("------------") ;
    println!("Vec length {}", vec2.len()) ;
    println!("Pop: {:?} ", vec2.pop()) ;
}