

fn main() {
    println!("Occupation is {} ", match get_occupation("Domenic"){
        Some(o) => o,
        None => "No occupation found",
    })
    
}


fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Domenic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _ => None,
    }
}


