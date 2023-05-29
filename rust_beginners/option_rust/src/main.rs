

#[derive(Debug, Eq, PartialEq)]
enum Access {
    Admin,
    User,
    Guest,
}

fn maybe_access(name: &str) -> Option<Access>{
    match name {
        "admin" =>  Some(Access::Admin) ,
        "gary" => Some(Access::User),
        _ => None,
    }
}


fn root() -> Option<Access>{
    Some(Access::Admin)
}



fn part_1() -> bool{
    // we are checking whether or not htis particular user has an access level.
    // the "Admin" user does have an access level
    // use is_some or is_none
    maybe_access("admin").is_some()
}


fn part_2() -> Option<Access> {
    //"root" is equivalent to Access:Admin , but it is not listed in the maybe_access function
    // use or_else and root()
    maybe_access("root").or_else(|| root())
}

fn part_3() -> Access {
    // "Alice" is not a listed user, so she will be a guest.Access
    // Note: Use unwrap_or_else
    maybe_access("Alice").unwrap_or_else(|| Access::Guest) 
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_part_1(){
        assert_eq!(part_1(), true, "Admins have an access level") ;
    }

    #[test]
    fn check_part2(){
        assert_eq!(
            part_2(),
            Some(Access::Admin),
            "Root users have Admin Access"
        )

    }

    #[test]
    fn ckeck_part_3(){
        assert_eq!(part_3(), Access::Guest, "Alice is a guest") ;
    }
}

fn main() {
    


}
