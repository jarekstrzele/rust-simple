

fn main() {
   
   /* replace */
   {
    let my_string = String::from("Rust is fantastic!") ;
    println!("After replace: {}", my_string.replace("fantastic", "great")) ;
   }

   /* lines */
   let my_string = String::from("The weather is\nnice\noutside mate") ;

   for line in my_string.lines() {
    println!("[ {} ]", line) ;

   }

   /* split */
   {
    let my_string = String::from("leave+a+like+if+you+enjoyed!") ;
    let tokens: Vec<&str> = my_string.split("+").collect() ;
    println!("At index 3: {}", tokens[2]) ;
   }

   /*trim */
   {
    let my_string = String::from("     My name is Domeniv    \n\r") ;
    println!("Before trim: {}", my_string);
    println!("After trim: {}", my_string.trim()) ; // doesn't change the my_string
    println!("Before after trim(): {}", my_string);
   }

   /* cars */
   {
    let my_string = String::from("decode on youtube") ;
    println!("{}", my_string) ;

    /* get character at index  */
    match  my_string.chars().nth(4) {
        Some(c) => println!("Char at index 4: {}", c ),
        None => println!("No character at index 4.")
    }
   }



}










