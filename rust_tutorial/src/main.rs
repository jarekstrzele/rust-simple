#![allow(unused)]

use std::io ;
use rand::Rng ;
use std::io::{Write, BufReader, BufRead, ErrorKind} ;
use std::fs::File ;
use std::cmp::Ordering ;


fn main() {
    
    println!("Max u32: {}", u32::MAX) ;
    println!("Max u64: {}", u64::MAX) ;
    println!("Max u128: {}", u128::MAX) ;
    println!("Max f32: {}", f32::MAX) ;
    println!("Max f128: {}", f64::MAX) ;
}