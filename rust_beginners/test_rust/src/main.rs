
fn all_caps(word: &str) -> String {
    word.to_uppercase()
}


fn main() { }
  

#[cfg(test)] // macro for testing modules
mod test {
    use crate::all_caps;
    #[test] // macro that tells that this fn test other code
    fn check_all_caps(){
        let result = all_caps("hello") ;
        let expected = String::from("HELLO");

        assert_eq!(result, expected, "string should be all uppercase") ;
    }
}


