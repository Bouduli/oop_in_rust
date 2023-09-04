
use std::{io, fmt::format, error::Error};

fn main(){

    clear();
    //Create a vector to store fruits;
    // let mut fruitVec:Vec<Fruit> = Vec::new();

    let mut loop_bool = true;
    while loop_bool{
        MainMenu('m');
        let mut input = String::new();


        io::stdin().read_line(&mut input).expect("Failed to read your line");
        let action = input.chars().nth(0).unwrap(); 
        loop_bool = MainMenu(action);
        
        io::stdin().read_line(&mut input);

    }
    //To make program not close as soon as instructions are done, it awaits user input before closing.

    

}

fn clear(){
    //Some black magic shii to clear the console
    print!("\x1B[2J\x1B[1;1H");
}

fn MainMenu(option: char) -> bool{
    // Function 

    //runs clear function 
    clear();

    
    //Program header. 
    println!("SMFOP - Super Magical Fruit Oop Program");

    //CONTEXT MENU ITEMS DETERMINED BY FUNCTION PARAMETER OPTION

    //main menu
    match option{
        'M'|'m'=>{println!("What do you want to do?\n- C/c: Create fruit, - R/r: Read all fruits, - U/u: Update a fruit, -D/d : Delete a fruit, -Q/q : Quit "); true}
        'C'|'c'=>create_fruit(),
        'R'|'r'=>read_fruits(),
        'U'|'u'=>update_fruits(),
        'D'|'d' =>delete_fruit(),
        'Q'|'q' => {println!("Goodbye"); false}
        _=>{println!("invalid inputs"); true}

    }

}


//Callback functions for CRUD related actions, Returns bool wether or not to continue the main loop; 
fn create_fruit() -> bool{

    //Colect the name of the fruit
    println!("Create Fruit\n------------\nWhat is the name of your fruit?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Your fruit name was invalid");
    
    //Collect the color of the fruit;
    println!("What color has your fruit\n--------------\n0 - Red, 1 - Green, 2 - Yellow, 3 - Orange");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read your line");
    let col:Color = Color::from_i32(input.trim().parse().unwrap()).unwrap();
    
    //Instatiate the fruit with collected parameters, 
    let newFruit = Fruit {
        name:name,
        color:col
    };

    println!("{}", newFruit.to_string());
    true
}
fn read_fruits()-> bool{
    println!("Read the fruits");
    true
}
fn update_fruits()-> bool{
    println!("Update a fruit");
    true
}
fn delete_fruit()-> bool{
    println!("Delete a fruit");
    true
}




//Fruit Related Shennanagains (?)
struct Fruit {
    name : String,
    color : Color,
    // has_wedges : bool,
    // has_core : bool
}
impl Fruit{
    fn eat_the_fruit(&self)-> String{
        "The fruit tastes nothing".to_string()
    }
    fn to_string(&self)->String{
        format!("{}: -Color:{}", self.name, self.color).to_string()
    }
}
enum Color{
    Red = 0,
    Green = 1,
    Yellow = 2, 
    Orange = 3
}
impl Color {
    fn from_i32(n:i32)->Result<Color, i32>{
        use Color::*;
        match n{
            0=>Ok(Red),
            1=>Ok(Green),
            2=>Ok(Yellow),
            3=>Ok(Orange),
            _=>Err(n)
        }
    }   
}
//Fuck this
impl std::fmt::Display for Color{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Color::Red=>Ok("Red"),
            Color::Green =>Ok("Green"),
            Color::Yellow=>Ok("Yellow"),
            Color::Orange=>Ok("Orange"),
            _=>Err(self)
        }
    }
}
