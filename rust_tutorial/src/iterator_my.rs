

pub fn iter_example(){
    let mut arr_it = [1,2,3] ;
    for val in arr_it.iter() {
        println!("{}", val) ;
    }

    // create your own iterator
    let mut iter1 = arr_it.iter() ;
    println!("1st: {:?} ", iter1.next()) ;
    println!("2nd: {:?} ", iter1.next()) ;
    println!("3rd: {:?} ", iter1.next()) ;
    println!("4th: {:?} ", iter1.next()) ;
}