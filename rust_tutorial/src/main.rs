#![allow(unused)]

// use std::io ;
// use rand::Rng ;
// use std::io::{Write, BufReader, BufRead, ErrorKind} ;
// use std::fs::File ;
// use std::cmp::Ordering ;
// mod my_casting; // import mdule
// use my_casting::{my_cast, Day} ; // use 
mod my_vec ;

use my_vec::vec_example ;


fn main() {
    vec_example();

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
