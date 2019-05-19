#![allow(warnings)]

//Used for standard input.
use std::io;

//Main function where the program starts
fn main()
{
    //Variable declarations of mutable new black strings as well as a boolean.
    let mut flag = String::new();
    let mut name = String::new();
    let mut power = String::new();
    let mut wpn = String::new();
    let mut worth = false;
    //A for loop to run 4 times to collect input
    for i in 0..4
    {
        //a simple match function to count as each input is put in
        match i
        {
            //if first input the test is entering the boolean variable which means
            //depending on if 1 or 0 set worth = true/false
            0 => {
                io::stdin().read_line(&mut flag).expect("Failed to read line");
                if flag == "1\n"
                {worth = true;}
                else if flag == "0\n"
                {worth = false;}
                else    //else print an error and break out if incorrect input
                {println!("Error: Input must be 1 (true) or 0 (false)."); assert!(false); break;}
            },
            1 => {io::stdin().read_line(&mut name).expect("Failed to read line");},     //read in the name of the hero
            2 => {io::stdin().read_line(&mut power).expect("Failed to read line");},    //read in the hero's power
            3 => {io::stdin().read_line(&mut wpn).expect("Failed to read line");},      //read in the hero's weapon
            _ => println!("out of bounds"),         //Catch anything that may go out of bounds
        }

    }


    //Now match on the name of the hero so we know what kind of struct we should create and what output to run.
    //Must add .trim() because "read_line" by default adds a newline to the end of the Input
    match name.trim()
    {
        //if name = Stan then struct = Stan
        "Stan" => {
            let mut the_man: Stan = Avenger::new(worth, name, power, wpn);  //Create a struct Stan with the gathered info
            the_man.i_am();             //Run the i_am Avenger trait function for the impl Avenger for Stan
            the_man.catch_phrase();     //Run the catch_phrase trait function
            the_man.change_worth();     //Change the worth or the Stan struct to being true or "Worthy"
        },
        //if name = Steve then struct = CaptainAmerica
        "Steve" => {
            let mut cap: CaptainAmerica = Avenger::new(worth, name, power, wpn);
            cap.i_am();
            cap.catch_phrase();
            cap.change_worth();
        },
        //if name = Tony then struct = IronMan
        "Tony" => {
            let mut iron: IronMan = Avenger::new(worth, name, power, wpn);
            iron.i_am();
            iron.catch_phrase();
            iron.change_worth();
        },
        //if name = Bruce then struct = Hulk
        "Bruce" => {
            let mut hulk: Hulk = Avenger::new(worth, name, power, wpn);
            hulk.i_am();
            hulk.catch_phrase();
            hulk.change_worth();
        },
        //if name = Thor then struct = Thor
        "Thor" => {
            let mut thor: Thor = Avenger::new(worth, name, power, wpn);
            thor.i_am();
            thor.catch_phrase();
            thor.change_worth();
        },
        _ => assert!(false) //Else the inputted name isn't valid for the selection of heroes
    }

}

//Struct definitions for all of the possible super heroes.
pub struct Stan { excelsior: bool, name: String, superpower: String, _weapon: String }

pub struct Thor { excelsior: bool, name: String, superpower: String, _weapon: String }

pub struct IronMan { excelsior: bool, name: String, superpower: String, _weapon: String }

pub struct Hulk { excelsior: bool, name: String, superpower: String, _weapon: String }

pub struct CaptainAmerica { excelsior: bool, name: String, superpower: String, _weapon: String }


//Public Avenger trait should be inheritted by every Avenger in order to save the world.
pub trait Avenger
{
    //Definition of the new Avenger trait which is called upon the creation of the struct trait.
    fn new(excelsior: bool, name: String, superpower: String, _weapon: String) -> Self;

    //A worthy function that returns the boolean variable of the trait
    fn worthy(&self) -> bool;

    //Default function to print that can be overridden depending which hero is using the trait
    fn i_am(&self)
    {println!("Wait....who am I?");}

    //Default function to print that can be overridden depending which hero is using the trait
    fn catch_phrase(&self)
    {println!("What is a catch phrase?");}
}

//Implementing the Stan struct. This is the area where the functions specific to this struct are defined
impl Stan
{
    //A function to return the structs boolean variable
    fn is_excelsior(&self) -> bool
    {self.excelsior}

    //the change worth function that prints out based upon the return of the is_excelsior function.
    //if the hero is worthy it says so else it says in all due time and changes them to being worthy
    fn change_worth(&mut self)
    {
        if self.is_excelsior()
        {println!("{} is always worthy...", self.name.trim());}
        else
        {
            println!("In all due time. A hero becomes more than a hero.");
            self.excelsior = true;
        }
    }
}
//Implementing the Thor struct
impl Thor
{
    fn is_excelsior(&self) -> bool
    {self.excelsior}

    fn change_worth(&mut self)
    {
        self.excelsior = true;
        println!("{} is always worthy...", self.name.trim());
    }
}

//Implementing the IronMan struct
impl IronMan
{
    fn is_excelsior(&self) -> bool
    {self.excelsior}

    fn change_worth(&mut self)
    {
        if self.is_excelsior()
        {println!("{} is already worthy...", self.name.trim());}
        else
        {
            println!("In all due time. A hero becomes more than a hero.");
            self.excelsior = true;
        }
    }
}

//Implementing the Hulk struct
impl Hulk
{
    fn is_excelsior(&self) -> bool
    {self.excelsior}

    fn change_worth(&mut self)
    {
        if self.is_excelsior()
        {println!("{} is already worthy...", self.name.trim());}
        else
        {
            println!("In all due time. A hero becomes more than a hero.");
            self.excelsior = true;
        }
    }
}

//Implementing the CaptainAmerica struct
impl CaptainAmerica
{
    fn is_excelsior(&self) -> bool
    {self.excelsior}

    fn change_worth(&mut self)
    {
        if self.is_excelsior()
        {println!("{} is already worthy...", self.name.trim());}
        else
        {
            println!("In all due time. A hero becomes more than a hero.");
            self.excelsior = true;
        }
    }
}

//Implementing the Avenger trait for the specific struct Stan
impl Avenger for Stan
{
    //A somewhat constructor of the new trait where the Stan struct is being constructed with the passed parameters
    //from the main()
    fn new(excelsior: bool, name: String, superpower: String, weapon: String) -> Stan
    {Stan { name: name, excelsior: excelsior,  superpower: superpower, _weapon: weapon}}

    //The definition of the worthy function declared in the original Avenger trait
    fn worthy(&self) -> bool
    {self.is_excelsior()}

    //The overriding of the i_am function declared in the original Avenger trait
    fn i_am(&self)
    {println!("My name is {}, my power is {}.", self.name.trim(), self.superpower.trim());}

    //The overriding of the catch_phrase function declared in the original Avenger trait
    fn catch_phrase(&self)
    {println!("With great power, comes great responsibility.");}
}

//Implementing the Avenger trait for the specific struct Thor
impl Avenger for Thor
{
    fn new(excelsior: bool, name: String, superpower: String, weapon: String) -> Thor
    {Thor { name: name, excelsior: excelsior,  superpower: superpower, _weapon: weapon}}

    fn worthy(&self) -> bool
    {self.is_excelsior()}

    fn i_am(&self)
    {println!("My name is {}, my power is {} and my weapons are {}.", self.name.trim(), self.superpower.trim(), self._weapon.trim());}

    fn catch_phrase(&self)
    {println!("You're big. I've fought bigger.");}
}

//Implementing the Avenger trait for the specific struct IronMan
impl Avenger for IronMan
{
    fn new(excelsior: bool, name: String, superpower: String, weapon: String) -> IronMan
    {IronMan { name: name, excelsior: excelsior,  superpower: superpower, _weapon: weapon}}

    fn worthy(&self) -> bool
    {self.is_excelsior()}

    fn i_am(&self)
    {println!("My name is {}, my power is {} and my weapon is {}.", self.name.trim(), self.superpower.trim(), self._weapon.trim());}

    fn catch_phrase(&self)
    {println!("I am Iron Man. *snap* ");}
}

//Implementing the Avenger trait for the specific struct Hulk
impl Avenger for Hulk
{
    fn new(excelsior: bool, name: String, superpower: String, weapon: String) -> Hulk
    {Hulk { name: name, excelsior: excelsior,  superpower: superpower, _weapon: weapon}}

    fn worthy(&self) -> bool
    {self.is_excelsior()}

    fn i_am(&self)
    {println!("My name is {}, my power is {} and my weapon is {}.", self.name.trim(), self.superpower.trim(), self._weapon.trim());}

    fn catch_phrase(&self)
    {println!("HULK SMASH!");}
}

//Implementing the Avenger trait for the specific struct CaptainAmerica
impl Avenger for CaptainAmerica
{
    fn new(excelsior: bool, name: String, superpower: String, weapon: String) -> CaptainAmerica
    {CaptainAmerica { name: name, excelsior: excelsior,  superpower: superpower, _weapon: weapon}}

    fn worthy(&self) -> bool
    {self.is_excelsior()}

    fn i_am(&self)
    {println!("My name is {}, my power is {}, my weapon is {}.", self.name.trim(), self.superpower.trim(), self._weapon.trim());}

    fn catch_phrase(&self)
    {println!("I could do this all day.");}
}
