use std::{io::{self, BufRead}, path::PathBuf, fs};
use structopt::{StructOpt};

const WORDLIST:&str = include_str!("./meaningfullwordlist.txt");

#[derive(StructOpt, Debug)]
#[structopt(name = "meaningcheck")]
struct Opt {
    /// Use this to meaning check a file
    #[structopt(short, long, parse(from_os_str))]
    file: Option<PathBuf>,

    /// The threshold for meaning checking
    #[structopt(short, long, default_value = "0.3")]
    threshold: f32,
}

fn main() {
    let opt = Opt::from_args();
    let threshold: f32 = opt.threshold;

    if opt.file.is_none() {
        let input = get_input();
        print_meaningful_lines(&input, &threshold);
    } else {
        let input = fs::read_to_string(opt.file.unwrap()).unwrap();
        print_meaningful_lines(&input, &threshold);
    }
}

fn get_input() -> String {
    let input = io::stdin().lock().lines().fold("".to_string(), |acc, line| {
        acc + &line.unwrap() + "\n"
    });
    return input;
}

fn print_meaningful_lines(input: &String, threshold: &f32) {
    let lines: Vec<&str> = input.split('\n').collect();
    let meaningfulwords: Vec<&str> = WORDLIST.split('\n').collect();

    for line in lines {
        if line_has_meaning(&line.to_lowercase(), &meaningfulwords, threshold) {
            println!("{line}");
        }
    }
}

fn line_has_meaning(line: &str, meaningfulwords: &Vec<&str>, threshold: &f32) -> bool {
    let mut meaning_lenght = 0;
    for word in meaningfulwords {
        if line.contains(word) {
            meaning_lenght += word.len();
        }
    }

    if (meaning_lenght as f32 / line.len() as f32) > *threshold {
        return true;
    }

    return false;
}
