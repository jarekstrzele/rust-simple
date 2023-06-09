#![allow(unused)]

// use std::io ;
// use rand::Rng ;
// use std::io::{Write, BufReader, BufRead, ErrorKind} ;
// use std::fs::File ;
// use std::cmp::Ordering ;
// mod my_casting; // import mdule
// use my_casting::{my_cast, Day} ; // use 
// mod my_vec ;
// use my_vec::vec_example ;
// mod functions_my;
// use functions_my::function_examples ;
mod generics_my;
use generics_my::generics_examples;

mod hashmap_my;
use hashmap_my::hashmap_example;

mod restaurant;
use crate::restaurant::order_food;

mod errors_my ;

mod iterator_my;
use iterator_my::iter_example;

mod closure_my;
// use closure_my::closure_example ;
mod box_my;

mod concurrency;
fn main() {
    concurrency::tread_example();
    
    //box_my::box_example();

    //closure_my::closure_example();
   
   // iter_example() ;

    //generics_examples();
    //order_food() ;

    // errors_my::errors_example() ;


//    let str1 = String::from("world") ;
//    let str2 = str1.clone();
//    println!("Hello {str1}") ;
//    println!("Hello {}", &str1) ;
   // generics_my::generics_examples();
    // my_cast() ;
    // let today: Day = Day::Monday ;
    // println!("{:?}", today) ;
    // println!("Is today the weekend {}", today.is_weekend()) ;

//     let random_num: i32 = rand::thread_rng().gen_range(1..6+1) ;
//     println!("random number {:?}", random_num) ;
   
//     let st3: String = String::from("x r t b h k k a m c") ;
//     let mut v1: Vec<char> = st3.chars().collect() ;
//     println!("{:?}", st3.chars()) ;
//     v1.sort();
//     v1.dedup(); // delete duplicate
//     for char in v1 {
//         print!("{char}") ;
//     }
//     let st4: &str = "Random string" ;
//     let mut st5: String = st4.to_string();
//     println!("{}", st5);
//     let byte_arr1: &[u8] = st5.as_bytes() ;
//     println!("byte arr {:?}", byte_arr1) ;
//     let st6: &str = &st5[0..6] ;
//     println!("st6 {:?}", st6) ;
//     println!("string length : {}", st6.len()) ;

//     st5.clear();
//     let st6: String = String::from("Just some ") ;
//     let st7: String = String::from(" words") ;
//     let st8: String = st6 + &st7 ;
//     for char in st8.bytes(){
//         print!("{}", char) ;
//     }
//     println!("\n{st8}") ;
//    // println!("\n{st6}") ; // generates some errors


}
