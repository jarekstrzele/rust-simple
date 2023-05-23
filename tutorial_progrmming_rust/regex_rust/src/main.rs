use regex::Regex;

fn main() {

    // \d - single digit
    // \w{5}  - five leters word
    let re = Regex::new(r"\w{5}").unwrap() ; // new() returns Result ( Ok(), Err() ); // unwrap() get the actual reg extract itself
    let text = "dcode" ;

    println!("Found match? {}", re.is_match(text)) ;

    //re.captures() returns Option (Some(), None)
    match re.captures(text) {
        // caps.get(0) - getting entire match
        // unwrap() - to get the struct itself
        //as_str() - to conver to string
        // Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
        // to achiev the same result you can write &caps[0]
        Some(caps) => println!("Found match: {}", &caps[0]),
        None => println!("Could not find match ..."),
    }

}