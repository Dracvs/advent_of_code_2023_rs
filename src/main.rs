use std::io;
use crate::aoc_01_trebuchet::the_trebouchet;
mod aoc_01_trebuchet;

fn main() {
    println!("Advent of Code 2023");
    println!("Choose your excercise:
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
