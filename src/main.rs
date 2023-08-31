
use std::io;

fn main(){

    clear();
    
    let mut loopBool = true;
    while loopBool{
        clearOptions('m');

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your line");
        
        println!("You wrote: {}", input);


    }
}

fn clear(){
    //Some black magic shii to clear the console
    print!("\x1B[2J\x1B[1;1H");

    //Program header. 
    println!("SMFOP - Super Magical Fruit Oop Program");

    
}

fn clearOptions(option: char){
    // Function 

    //runs clear function 
    clear();

    //CONTEXT MENU ITEMS DETERMINED BY FUNCTION PARAMETER OPTION

    //main menu
    match option{
        'M'|'m'=>println!("What do you want to do?\n- C/c: Create fruit, - R/r: Read all fruits, - U/u: Update a fruit, -D/d : Delete a fruit, -Q/q : Quit "),
        'C'|'c'=>println!("Create Fruit\nWhat fruit did you wish to make?"),
        'R'|'r'=>println!("Read the fruits"),
        'U'|'u'=>println!("Update a fruit"),
        'D'|'d' => println!("Delete a fruit"),
        _=>println!("fuck you"),

    }

}
struct Fruit {
    name : String,
    color : Color,
    has_wedges : bool,
    has_core : bool
}
impl Fruit{
    fn eat_the_fruit(&self)-> String{
        "The fruit tastes nothing".to_string()
    }
}

enum Color{
    Red,
    Green,
    Yellow, 
    Orange
}