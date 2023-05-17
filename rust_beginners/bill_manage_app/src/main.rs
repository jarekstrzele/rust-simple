use std::io ;


fn get_input() -> io::Result<String> {
    let mut buffer = String::new() ;
    io::stdin().read_line(&mut buffer)

}


fn main() {
    println!("Hello, world!");
}
