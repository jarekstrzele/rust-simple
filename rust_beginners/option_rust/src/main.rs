fn main() {
    let a: Option<i32> = None ;
    dbg!(a) ;
    
    let a_is_some  = a.is_some() ; //true/false
    dbg!(a_is_some) ;
    
    let a_is_none  = a.is_none() ;
    dbg!(a_is_none) ;

    let a_mapped  = a.map(|num| num + 1) ; //if there is no data, it will be no error
    dbg!(a_mapped) ;

    let a_filtered = a.filter(|num| num == &1) ; //filter borrows the number, so you need to borrow your comparison
    dbg!(a_filtered) ;
    
    let a_or_else = a.or_else(| | Some(5)) ; //if a is a data->nothing, if a is not a data->Some(5); it returns Option
    dbg!(a_or_else) ;
    
    // if a has a data, this data will be placed within the variable
    // else 0 will be place within the variable
    let unwrapped = a.unwrap_or_else(|| 0) ; // it takes out the data and then place it within a variable
    dbg!(unwrapped) ;



}
