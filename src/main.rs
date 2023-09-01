
use std::io;

fn main(){

    clear();

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
    println!("Create Fruit\n------------\nWhat is the name of your fruit?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Your fruit name was invalid");
    
    println!("What color has your fruit\n--------------\n0 - Red, 1 - Green, 2 - ");

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