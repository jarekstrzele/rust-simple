pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2) 
}
    
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests_2 {

    // the child module has access to anything in their parent
    // event private fields
    use super::* ; // use fn add_two(), interma;_adder

    #[test]
    fn internal(){
        assert_eq!(4, internal_adder(2,2)) ;
    }
    
}

