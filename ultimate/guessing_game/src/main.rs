use std::io ;
use rand::Rng;
use std::cmp::Ordering ;

fn main() {
    println!("Guess the number!");

    //`thread_rng()` - gives us a random number generator
    let secret_number = rand::thread_rng().gen_range(1, 101) ;
    println!("The secret number is: {}", secret_number) ;

    loop {
        println!("Please input your guess.") ;

        let mut guess: String = String::new(); //`new()` static method returnsan empty string that we could use
        
    
        // it will take a reference to string , modify it without taking ownership of the string
        // return a `Result` type
        io::stdin().read_line(&mut guess).expect("Failed to read line") ;
        //`parse` returns a Result type
        //let guess: u32 = guess.trim().parse().expect("Please type a number!") ;
        let guess: u32 = match guess.trim().parse() {
            Ok(num: u32) => num,
            Err(_) => continue,

        }
       
        println!("You guessed: {}", guess) ;
    
        // cmp returns a Ordering ype value
        match guess.cmp(&secret_number){
            Ordering::Less => println!("TO small!") ,
            Ordering::Greater => println!("Too big!") ,
            Ordering::Equal => {
                println!("You win!");
                break ;
            }
        }
    

    }
   

}
