use std::net::TcpListener ;


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self{
        Self { 
            addr: addr 
        }
    }

    pub fn run(self){
        println!("Listening on {}", self.addr) ;

        //let listener = TcpListener::bind(&self.addr) ; //it wraps listener into Result
        // so you have to unwrap it to take w listener
        let listener = TcpListener::bind(&self.addr).unwrap() ;
    
    }
}