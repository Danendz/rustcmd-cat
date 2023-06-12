use std::collections::binary_heap::Iter;
use std::collections::HashSet;
use std::{env, process::exit};

#[derive(Debug, PartialEq, Eq, Hash)]
enum CommandOptions {
    ShowEnds,
    Number,
    Help,
}

impl TryFrom<&str> for CommandOptions {
    type Error = ();

    fn try_from(cmd: &str) -> Result<Self, Self::Error> {
        match cmd {
            cmd if cmd == "-E" || cmd == "--show-ends" => Ok(CommandOptions::ShowEnds),
            cmd if cmd == "-n" || cmd == "--number" => Ok(CommandOptions::Number),
            cmd if cmd == "-h" || cmd == "--help" => Ok(CommandOptions::Help),
            &_ => {
                eprintln!("ERROR: Couldn't find command: {cmd}");
                println!("To get all available commands use: -h or --help");
                exit(1);
            }
        }
    }
}

impl CommandOptions {
    fn help() -> () {
        println!("\ncat\n");
        println!("Concatenate and print (display) the content of files.\n");
        println!("Syntax\n");
        println!("\tcat [Options] [File]...\n");
        println!("Concatenate FILE(s), or standard input, to standard output.\n");

        let commands = ["ShowEnds", "Number", "Help"];

        fn make_command_string(command: &str, description: &str) -> String {
            format!("\t{command}\t\t{description}\n")
        }

        for command in commands {
            match command {
                command if command == "ShowEnds" => println!(
                    "{str}",
                    str = make_command_string("-E, --show-ends", "display $ at end of each line")
                ),
                command if command == "Number" => println!(
                    "{str}",
                    str = make_command_string("-n, --number", "number all output lines")
                ),
                command if command == "Help" => println!(
                    "{str}",
                    str = make_command_string("-h, --help", "display this help and exit")
                ),
                &_ => unreachable!("Couldn't find command: {command}"),
            }
        }

        exit(1)
    }
}

fn read_args() -> (HashSet<CommandOptions>, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let mut options: HashSet<CommandOptions> = HashSet::new();
    let mut file_paths: Vec<String> = vec![];

    for i in 1..args.len() {
        let arg = args[i].clone();
        if arg.starts_with("-") == false {
            file_paths.push(arg);
        } else {
            let cmd_option = CommandOptions::try_from(arg.as_str());
            match cmd_option {
                Ok(c) => {
                    match c {
                        CommandOptions::Help => CommandOptions::help(),
                        _ => { options.insert(c); }
                    };
                }
                Err(_) => (),
            };
        }
    }

    return (options, file_paths);
}

fn display_args(options: &HashSet<CommandOptions>, file_paths: &Vec<String>) -> () {
    println!("Options: {:?}, File paths: {:?}", options, file_paths)
}

fn main() {
    let (options, file_paths) = read_args();
    display_args(&options, &file_paths);
}
