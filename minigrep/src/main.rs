use std::env;
use std::fs;
use std::process; // to exit a program without panicing

fn main() {
   let args: Vec<String> = env::args().collect() ;
  
   let config = Config::new(&args) ;

   println!("Searching for  {}", config.query) ;
   println!("in file {}", config.filename) ;

   let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file") ;
   println!("With text: \n{}", contents) ;

}

// to link query with filename
struct Config{
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            panic!("not enough arguments") ;
        }
        let query = args[1].clone() ;
        let filename = args[2].clone() ;
    
        Ok(Config { query, filename })
    }
}
