use termion::color;

use std::env;
use std::process::exit;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let mut flags = String::new();

    if argc < 3 {
        // Exit if less than two arguments are supplied
        eprintln!(
            "{}ERROR: Please enter two (2) arguments{}",
            color::Fg(color::Red),
            color::Fg(color::Reset)
        );
        exit(1);
    }
    if argc > 3 {
        flags = String::from(&argv[3]);
    }

    let search = &argv[1];
    let file = &argv[2];

    let file_lines_str = std::fs::read_to_string(file).expect("Could not read file.");
    let file_lines = file_lines_str.split('\n');

    let mut uncontained = String::new();
    let mut contained = String::new();

    for line in file_lines {
        let contains_string: bool;

        if !flags.contains('i') {
            contains_string = line.contains(search);
        } else {
            contains_string = line.to_lowercase().contains(&search.to_lowercase());
        }

        if !contains_string {
            uncontained = format!("{}\n{}", uncontained, line);
        } else {
            contained = format!("{}\n{}", contained, line);
        }
    }

    if flags.contains('u') {
        for (line_number, line) in uncontained.split('\n').enumerate() {
            if line_number != 0 {
                println!("{}: {}", line_number, line);
            }
        }
    } else {
        for (line_number, line) in contained.split('\n').enumerate() {
            if line_number != 0 {
                println!("{}: {}", line_number, line);
            }
        }
    }
}
