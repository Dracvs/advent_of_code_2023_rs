use std::io;

#[path = "./day_01/trebuchet.rs"]
mod trebouchet;

use trebouchet::the_trebouchet;

fn main() {
    println!("
        Advent of Code 2023");
    println!("
        Choose your excercise: 
        1: The Trebuchet Sequence");

    let mut activity = String::new();
    io::stdin()
        .read_line(&mut activity)
        .expect("Failed to read line");
    
    if activity.trim().eq_ignore_ascii_case("1")
    {
        the_trebouchet();
    }
    else {
        return
    }
}
