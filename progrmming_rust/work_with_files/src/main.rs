use std::fs::File ;
use std::io::prelude::* ;



fn main() {
    let mut file = File::open("info.txt").expect("Cant' opent file!") ;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Oops! Can not reat the file...") ;

    println!("File Contetns:\n\n{}", contents) ;



}
