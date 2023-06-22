use std::io::{self, BufRead};

const WORDLIST:&str = include_str!("./meaningfullwordlist.txt");
const THRESHHOLD: f32 = 0.3;

fn main() {
    let input = get_input();
    print_meaningful_lines(&input);
}

fn get_input() -> String {
    let input = io::stdin().lock().lines().fold("".to_string(), |acc, line| {
        acc + &line.unwrap() + "\n"
    });
    return input;
}

fn print_meaningful_lines(input: &String) {
    let lines: Vec<&str> = input.split('\n').collect();
    let meaningfulwords: Vec<&str> = WORDLIST.split('\n').collect();

    for line in lines {
        if line_has_meaning(line, &meaningfulwords) {
            println!("{line}");
        }
    }
}

fn line_has_meaning(line: &str, meaningfulwords: &Vec<&str>) -> bool {
    let mut meaning_lenght = 0;
    for word in meaningfulwords {
        if line.contains(word) {
            meaning_lenght += word.len();
        }
    }

    if (meaning_lenght as f32 / line.len() as f32) > THRESHHOLD {
        return true;
    }

    return false;
}
