use std::collections::HashMap ;

pub fn hashmap_example(){
    let mut heroes = HashMap::new() ;
    heroes.insert("Superman", "Clark Kent") ;
    heroes.insert("Batman", "Bruce Wayne") ;
    heroes.insert("The Flash", "Barry Allen") ;

    for (key, value) in heroes.iter(){
        println!("{} = {}", key, value) ;

    }
    println!("length {}", heroes.len()) ;


    if heroes.contains_key(&"Batman"){
        let the_batman = heroes.get(&"Batman");
        println!("the batman {:?}", the_batman) ;

        match the_batman {
            Some(x) => println!("Batmanis a hero!!"),
            None => println!("Batman is not a hero"),
        }
    }
}






