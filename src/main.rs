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

    let mut line_count = 1;

    for line in file_lines {
        let sorted_line: String; // New string holder to accomidate for case-insensitive
        let sorted_search: String; // New string holder to accomidate for case-insensitive

        line_count += 1;

        if flags.contains('i') {
            sorted_line = line.to_lowercase();
            sorted_search = search.to_lowercase();
        } else {
            sorted_line = String::from(line);
            sorted_search = String::from(search);
        }

        if !line.contains(&sorted_search) {
            uncontained = format!("{}\n{}: {}", uncontained, line_count, sorted_line);
        } else {
            contained = format!("{}\n{}: {}", contained, line_count, sorted_line);
        }
    }
    if flags.contains('u') {
        print!("{}", uncontained);
    } else {
        print!("{}", contained);
    }
    println!();
}
