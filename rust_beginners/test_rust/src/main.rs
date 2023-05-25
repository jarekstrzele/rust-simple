
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if  n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}


fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }else {
        Some(a/b)
    }
}

fn concat(first: &str, second: &str) -> String{
    format!("{}{}", first, second)
}


fn main() { }
  

#[cfg(test)]
mod test {
    use crate::* ;

    #[test]
    fn clamp_lower(){
        let result = clamp(10,100,1000);
        let expeted = 100;
        assert_eq!(result, expeted, "should br 100") ;
    }

    #[test]
    fn clamp_upper(){
        let result = clamp(5000,100,1000);
        let expeted = 1000;
        assert_eq!(result, expeted, "should br 1000") ;
    }

    #[test]
    fn check_div(){
        let result = div(1,1);
        let expected = Some(1) ;
        assert_eq!(result, expected, "should be 1") ;

    }

    #[test]
    fn check_div_zero(){
        let result = div(1,0);
        let expected = None;
        assert_eq!(result, expected, "connot divide by 0");
    }

    #[test]
    fn check_concat(){
        let result = concat("a", "b") ;
        let expected = String::from("ab");
        assert_eq!(result,expected, "should be ab") ; 
    }

}

