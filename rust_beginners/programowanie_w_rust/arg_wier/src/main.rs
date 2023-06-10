//importuje do bieżącego zasięgu zestaw metod  (trait)
// zestaw to kolekcja metod
use std::str:FromStr;

//oferuje kilka przydatnych funkcji i typów
//np.  funkcja `args`, zapewnia dostęp do args wiersza poleceń
use std::env;


fn main() {
    let mut numbers = Vec::new() ; // pusty wektor, jak lista w Pythonie, tablica w JS


    for arg in std::env::args().skip(1){ // funkcja 1args zwraca iterator, który pozwala
        // uzyskać dostęp do  kolejnych argumentów i w tej kolejności je petwarzać
        numbers.push(u64::from_str(&arg)
            .expect("Błąd parsowania argumentu")) ;
    }
}
