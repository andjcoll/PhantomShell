// Shell version 0.2.0
// Operating Systems - Andrew Collins

pub mod vfs;

use clap::{error::ErrorKind, ArgAction, CommandFactory};
use std::{
    fs::read_to_string,
    io::{self, BufRead, IsTerminal},
};
use vfs::vfs::VFS;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "shell",
    about = "Processes a virtual file system in a custom format from a file or stdin.",
    version = "0.2.0",
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Args {
    /// Input file (-f filename)
    #[arg(short = 'f', long)]
    file: Option<String>,

    /// Print help
    #[arg(short = 'h', long, short_alias = '?', action = ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'v', long, action = ArgAction::Version)]
    version: Option<bool>,

    /// Lists all files on the disk
    #[arg(long, action = ArgAction::SetTrue)]
    dir: bool,
}

fn main() {
    let args = Args::parse();
    let mut command = Args::command();

    let input = if let Some(file) = args.file {
        match read_to_string(&file) {
            Ok(content) => content,
            Err(_) => {
                command
                    .error(ErrorKind::Io, format!("File {} not found", file))
                    .exit();
            }
        }
    } else if io::stdin().is_terminal() {
        command.print_help().unwrap();
        return;
    } else {
        read_from_stdin()
    };

    let vfs = match VFS::parse(&input) {
        Some(vfs) => vfs,
        None => {
            command
                .error(ErrorKind::ValueValidation, "Invalid VFS parse")
                .exit();
        }
    };

    if args.dir {
        for file in vfs.get_files() {
            println!("{}", file);
        }
    } else {
        print!("{}", vfs.print());
    }
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
