// Shell version 0.1.0
// Operating Systems - Andrew Collins

use std::{
    env,
    fs::read_to_string,
    io::{self, BufRead, IsTerminal},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we should print the help message
    if args.iter().any(|a| a == "-H" || a == "-h" || a == "-?") {
        print_help();
        return;
    }

    // Check if we should print the version number
    if args.iter().any(|v| v == "-v" || v == "-V") {
        print_version();
        return;
    }

    // Get input from either stdin or filein
    let input = if args.len() <= 1 {
        // Is a terminal, print help
        if io::stdin().is_terminal() {
            print_help();
            return;
        } else {
            let input = read_from_stdin();
            input.to_string()
        }
    } else {
        let filename = args[1].clone();
        let res = read_to_string(&filename);
        if !res.is_ok() {
            eprintln!("File {} does not exist", filename);
            return;
        }

        res.unwrap()
    };

    // remove and re-add the labels from the input to prove we can
    let stripped = remove_labels(input);
    let added = add_labels(stripped);
    print!("{}", added)
}

// Reads a file from stdin (from pipe)
fn read_from_stdin() -> String {
    let mut input = String::new();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
        input.push_str("\n");
    }
    input.pop();
    input
}

// Prints the help message
fn print_help() {
    println!("Usage: shell [INPUT_FILE]");
    println!("Can also piped into standard in.");
    println!();
    println!("Options:");
    println!("  -h, -H, -?: Prints this help message");
    println!("  -v: Prints version number");
}

// Prints the version number
fn print_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Shell Version: {}", version);
}

// Removes the labels from the file system
// assumes 2 hex digits
fn remove_labels(file_system: String) -> String {
    let mut removed = String::new();
    for line in file_system.lines().skip(2) {
        if line.len() <= 3 {
            eprintln!("Malformatted disk. Line isn't long enough.");
            exit(1);
        }
        removed.push_str(&line[3..]);
        removed.push_str("\n");
    }
    removed.to_string()
}

// Adds the label back
fn add_labels(file_system: String) -> String {
    let mut new_file_system = String::new();

    let mut tens = String::from("XX: ");
    let mut ones = String::from("XX:");

    // Create column numbering
    let first_line = file_system.lines().next();
    if !first_line.is_some() {
        eprintln!("Malformatted disk. Not enough rows.");
        exit(1);
    }
    let length = first_line.unwrap().len();
    let mut ten = 0;
    for i in 0..length {
        let one = i % 16;
        let hex = format!("{:X}", one);
        ones.push_str(&hex);

        if i != 0 && one == 0 {
            ten += 1;
            tens.push_str(&" ".repeat(15));
            let hex = format!("{:X}", ten);
            tens.push_str(&hex);
        }
    }

    new_file_system.push_str(&tens);
    new_file_system.push_str("\n");
    new_file_system.push_str(&ones);
    new_file_system.push_str("\n");

    // Create row numbering
    for (i, line) in file_system.lines().enumerate() {
        let label = format!("{:02X}:", i);
        new_file_system.push_str(&label);
        new_file_system.push_str(line);
        new_file_system.push_str("\n");
    }
    new_file_system.to_string()
}
