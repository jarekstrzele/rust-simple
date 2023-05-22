


use rand::Rng;


fn main() {
    // tworzenie generatora liczb losowych
    let mut rng = rand::thread_rng() ;
    let random_number: u32 = rng.gen_range(1..=6);
    println!("Random Number 1-100: {}", random_number) ;
    let random_number: u32 = rng.gen_range(1..6);
    println!("Random Number 1-99: {}", random_number) ;

    // generate random boolean
    // flip a coin
    let random_bool: bool = rng.gen() ; 
    println!("Random Number 1-99: {}", random_bool) ;
    // prawdopodobieństwo otrzymania prawdy
    let random_bool: bool = rng.gen_bool(0.7) ; // 70% for true, 30% for false
    println!("Random Number 1-99: {}", random_bool) ;
}
