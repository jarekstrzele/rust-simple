// Map Combinator

// - given a user name, create and print out a User struct if the user exists
// 
// find_user function to locate a user
// map function to create the User
// print out the User struct if found or "not found" if not

#[derive(Debug)]
struct User{
    user_id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name: String = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}



fn main() {
   let user_name = "sama" ;
   let user = find_user(user_name)
    .map(|user_id| 
        { 
            User{
                user_id,
                name: user_name.to_owned(),
            }
        }
    ) ;

   match user {
    Some(user) => println!("{:?}", user),
    None => println!("user not found"),
   }

}
