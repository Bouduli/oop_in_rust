
use std::io;

fn main(){
    let mut loopBool = true;
    while loopBool{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your line");

        println!("You wrote: {}", input);


    }
}

struct Fruit {
    name : String,
    color : Color,
    has_wedges : bool,
    has_core : bool
}
impl Fruit{
    fn eat_the_fruit(&self){ö
        "The fruit tastes nothing"
    }
}

enum Color{
    Red,
    Green,
    Yellow, 
    Orange
}