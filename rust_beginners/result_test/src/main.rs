enum Position {
    Maintenance,
    Manager,
    AssemblyTech,
    Marketing
} 

enum Status {
    Active,
    Terminated,
}
struct Employee{
    position: Position,
    status: Status,
}

fn may_enter_building(emp: &Employee) -> Result<(), String> {
    match emp.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),

    };

    match emp.position {
        Position::Manager => Ok(()),
        Position::Marketing => Ok(()),
        _ => Err("No access".to_owned()),
    }
}

fn print_access(emp: &Employee) -> Result<(), String> {
    let attemp_access = may_enter_building(emp)? ;
    println!("access ok") ;
    Ok(())
}

fn main(){
   let manager = Employee {
    position: Position::AssemblyTech,
    status: Status::Active,
   } ;

   match print_access(&manager) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
   }


}
