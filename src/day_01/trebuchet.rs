use std::env;
use std::fs;

/*
--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take 
a look. The Elves have even given you a map; on it, they've used stars to mark 
the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, 
you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each 
day in the Advent calendar; the second puzzle is unlocked when you complete 
the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") 
and where they're even sending you ("the sky") and why your map looks mostly blank 
("you sure ask a lot of questions") and hang on did you just say the sky 
("of course, where do you think snow comes from") when you realize that the 
Elves are already loading you into a trebuchet ("please hold still, we need to 
strap you in").

As they're making the final adjustments, they discover that their calibration 
document (your puzzle input) has been amended by a very young Elf who was apparently 
just excited to show off her art skills. Consequently, the Elves are having trouble 
reading the values on the document.

The newly-improved calibration document consists of lines of text; each line 
originally contained a specific calibration value that the Elves now need to 
recover. On each line, the calibration value can be found by combining the first 
digit and the last digit (in that order) to form a single two-digit number.
*/
pub fn the_trebouchet(){
    let file_path = "src/day_01/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    let string_array : Vec<&str> = contents.split("\n").collect();

    let mut number_array = Vec::<String>::with_capacity(string_array.clone().into_iter().count());
    let mut _counter: usize = 0;

    for word in string_array.clone(){
        let mut temporal = String::new();
        let mut wordy : Vec<char> = word.chars().collect();
        for index in wordy {
            if index.is_numeric() {
                temporal = index.to_string();
                break;
            }
        }
        wordy = word.chars().rev().collect();
        for index in wordy {
            if index.is_numeric() {
                temporal = temporal + &index.to_string();
                break;
            }
        }
        number_array.push(temporal);
        _counter += 1;
    }

    let mut summatory = 0;
    for sum in number_array {
        summatory +=  sum.parse::<i32>().unwrap();
    }
    println!("The Summatory does {}", summatory);
}