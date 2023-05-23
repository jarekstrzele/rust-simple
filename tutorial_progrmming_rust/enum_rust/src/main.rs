// remove any compile warrings
#![allow(dead_code)]

enum Day {
    Mon, Tue, Wed, Thur, Fri, Sat, Sun
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Sat | &Day::Sun => return false,
            _ => return true,
        }
    }
}


fn main() {
    let d = Day::Tue;

    println!("Is d a weekday {}", d.is_weekday()) ;
}
