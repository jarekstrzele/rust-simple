
#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    Yellow,
    White,

}

#[derive(Debug)]
struct ShoesColor(Color);

#[derive(Debug)]
struct ShirtColor(Color);

#[derive(Debug)]
struct PantsColor(Color);

impl ShoesColor {
    fn new(color: Color) -> Self{
        Self(color)
    }
}
impl ShirtColor {
    fn new(color: Color) -> Result<Self, String>{
        match color {
            Color::Purple => Err("Purple is not allowed".to_owned()),
            other => Ok(Self(other)),
        }
       
    }
}

impl PantsColor {
    fn new(color: Color) -> Self{
        Self(color)
    }
}

fn print_shoes_color(color: ShoesColor){
    println!("shirt color = {:?}", color) ;
}

fn print_shirt_color(color: Result<ShirtColor, String>){
    println!("shirt color = {:?}", color) ;
}

fn print_pants_color(color: PantsColor){
    println!("shirt color = {:?}", color) ;
}


fn main() {
    let shirt_color = ShirtColor::new(Color::Gray) ;
    let pants_color = PantsColor::new(Color::Blue) ;
    let shoes_color = ShoesColor::new(Color::White) ;
    
    print_shirt_color(shirt_color) ;
    print_pants_color(pants_color) ;
    print_shoes_color(shoes_color) ;
    
   
}
