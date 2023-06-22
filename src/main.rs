use std::io::{self, BufRead};

const WORDLIST:&str = include_str!("./meaningfullwordlist.txt");

fn main() {
    let input = get_input();
    let lines = input.split('\n');
    let meaningfullwords = WORDLIST.split('\n');

    for line in lines {
        println!("{line}");
    }
}

fn get_input() -> String {
    let input = io::stdin().lock().lines().fold("".to_string(), |acc, line| {
        acc + &line.unwrap() + "\n"
    });
    return input;
}


// if output_in_binary.find(stop_word_in_binary.as_str()) == None {
//     _error("Could not find an embedded message in the image");
//     exit(1);
// }
// output_in_binary = output_in_binary[..output_in_binary.find(stop_word_in_binary.as_str()).unwrap()].to_string();