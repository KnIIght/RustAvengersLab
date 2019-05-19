/*This piece of code tells the rustc compiler to allow warnings
I recommend keeping it but if you seem to have an issue you cannot
figure out, the rust compiler is EXTREMELY helpful in a lot of ways*/
#![allow(warnings)]

//This is your main function
fn main()
{
    println!("Welcome to the crew Avenger.")
}

//Public struct for Stan with set attributes
pub struct Stan { excelsior: bool, name: String, superpower: String, _weapon: String }

// Public Avenger trait should be inheritted by every Avenger in order to save the world.
pub trait Avenger
{
    fn new(/*Add parameters here based upon Stan*/) -> Self;

    // Traits can provide default method definitions.
    fn i_am(&self)
    {println!("Wait....who am I?");}

    // Write your catch phrase function.

    // Write your worthy function.
}

//Implementation of the struct Stan
//All Functions specific to Stan are defined here
impl Stan
{
    //Write function to return excelsior struct attribute

    //Function skeleton given to you.
    fn change_worth(&mut self)
    {
        //Your code goes here.
    }
}

//Implementing the Avenger trait specifically for Stan struct
impl Avenger for Stan
{
    fn new(/*Base hero parameters from the above Avenger trait*/) -> Stan
    {/*construct the Stan with given parameters.*/}

    fn i_am(&self)
    {/*Write this function specifically to Stan struct. What would Stan say?*/}

    // catch phrase function

    // worthy function returning excelsior's value
}
