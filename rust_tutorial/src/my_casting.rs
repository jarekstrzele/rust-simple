

pub fn my_cast(){

    let int_max: u8 = u8::MAX ;
    println!("{}", int_max) ;

    let int1_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int1_u8 as u32) 
                        + (int2_u8 as u32);

    println!("{int3_u32}")


}

#[derive(Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    pub fn is_weekend(&self) -> bool {
        match self{
            Day::Saturday | Day::Sunday => true,
            _ => false
        }
    }
}


