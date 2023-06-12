mod helper ;
mod group ;

pub fn print_from_lib(){
    println!("Hello from lib") ;
    // helper::print_from_helper() ;
    use helper::{print_from_helper, print_from_helper_again} ;
    print_from_helper();
    print_from_helper_again() ;
    group::g1::print_from_group_g1();

}