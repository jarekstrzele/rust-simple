use std::io ;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}


impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase() ;
        //as_str() takes String type and returns &strreboot

        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None
        }
    }
}

fn print_power_action(state: PowerState){
    // instead of that way
    // match state {
    //     PowerState::Off,
    //     PowerState::Sleep,
    //     PowerState::Reboot,
    //     PowerState::Shutdown,
    //     PowerState::Hibernate,
    // }
    use PowerState::* ;
    match state {
        Off => println!("turning off"),
        Sleep=> println!("sleeping"),
        Reboot=> println!("rebooting"),
        Shutdown=> println!("shutting down"),
        Hibernate=> println!("hibernating"),
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new() ;
    io::stdin().read_line(&mut buffer)? ;
    Ok(buffer.trim().to_owned()) 
}


fn main(){
    let mut buffer = String::new() ;

    println!("Enter new power state:") ;
    
    let user_input_status = io::stdin().read_line(&mut buffer) ;
    if user_input_status.is_ok() {
        match PowerState::new(&buffer) {
            Some(state) => print_power_action(state),
            None => println!("invalid power state"),
        } 
    }else {
        println!("error reading input") ;
    }

}