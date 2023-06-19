use std::collections::HashMap ;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main(){
    let mut  lockers = HashMap::new();
    lockers.insert(1, Contents {content: "stuff_1".to_owned()}) ;
    lockers.insert(2, Contents {content: "stuff_2".to_owned()}) ;
    lockers.insert(3, Contents {content: "stuff_3".to_owned()}) ;

    for (locker_num, content) in lockers.iter(){
        println!("key = {:?} , value = {:?}", locker_num, content)
    }
}


