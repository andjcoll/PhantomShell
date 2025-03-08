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
    #[arg(short = 'f', long, conflicts_with = "vfs_name")]
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

    /// Prints the contents of a file to standard output
    #[arg(long, alias = "type")]
    cat: Option<String>,

    /// Prints the disk usage
    #[arg(long, action = ArgAction::SetTrue)]
    du: bool,

    /// Name of the VFS to create
    #[arg(required = false, index = 1, conflicts_with = "file")]
    vfs_name: Option<String>,

    #[arg(required = false, index = 2, conflicts_with = "file")]
    rows: Option<usize>,

    #[arg(required = false, index = 3, conflicts_with = "file")]
    cols: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let mut command = Args::command();

    let flags = vec![args.du, args.cat.is_some(), args.dir];
    let active_flags = flags.iter().filter(|&&x| x).count();

    if active_flags > 1 {
        command
            .error(ErrorKind::ArgumentConflict, "Only use one command")
            .exit();
    }

    if let (Some(vfs_name), Some(size_one), Some(size_two)) = (&args.vfs_name, args.rows, args.cols)
    {
        if size_two % 2 != 0 {
            command
                .error(
                    ErrorKind::ValueValidation,
                    "cols value must be divisible by 2",
                )
                .exit();
        }

        let vfs = VFS::new(&vfs_name, size_one, size_two);
        match vfs {
            Ok(vfs) => println!("{}", vfs.print()),
            Err(err) => command.error(ErrorKind::ValueValidation, err).exit(),
        }
        return;
    } else if args.vfs_name.is_some() || args.rows.is_some() || args.cols.is_some() {
        command
            .error(
                ErrorKind::ValueValidation,
                "All three arguments (vfs_name, rows, cols) must be provided together.",
            )
            .exit();
    }

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
            println!("{}", file.0);
        }
    } else if let Some(file) = args.cat {
        let out = vfs.cat_file(&file);
        match out {
            Ok(out) => println!("{}", out),
            Err(err) => command.error(ErrorKind::ValueValidation, err).exit(),
        }
    } else if args.du {
        let disk_usage = vfs.disk_usage();
        println!("{}", disk_usage);
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
