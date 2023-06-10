struct Person {
    name: String,
    age: u8,
}

trait HasVoiceBox {
    //speak
    fn speak(&self) ;

    //check if can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self){
        println!("Hello, my name is {}", self.name) ;
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}


fn main() {
    let p = Person {
        name: String::from("Bob"),
        age: 10,
    } ;

    println!("Can {} speak? {}", p.name, p.can_speak());
    p.speak() ;
}
