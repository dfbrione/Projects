mod helpers;
mod date;
mod misc;
use misc:: printMe;
use date:: *;
use helpers:: *;
use std::io;

fn main() {

    //TEST DATE STRUCT/CLASS

    /* 
    let mut sample_date = date::Date::new();
    println!("What type of date would you like to print? ");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input= input.trim(); //Remove whitespace at the end
    sample_date.print_date(&input);

    */

    misc::printMe();
    let sample_str = "Hello";
    println!("{}", sample_str.chars().count());
    sample_str.chars().nth()
    

    //TEST PERSON
}
