struct User {
    user_id: i32,
    name: String,
}


fn find_user(name: &str) -> Option<i32> {
    //let name shadows name parameter
    // to_lowercase returns String
    let name = name.to_lowercase();

    //as_str() changes String -> &str
    match name.as_str() {
        "sam" => Some(1) ,
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}


fn main() {
    
}
