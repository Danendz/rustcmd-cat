use std::collections::HashSet;
use std::fs;
use std::{env, process::exit};

#[derive(Debug, PartialEq, Eq, Hash)]
enum CommandOptions {
    NumberNonBlank,
    ShowEnds,
    Number,
    ShowTabs,
    Help,
}

impl TryFrom<&str> for CommandOptions {
    type Error = ();

    fn try_from(cmd: &str) -> Result<Self, Self::Error> {
        match cmd {
            cmd if cmd == "-E" || cmd == "--show-ends" => Ok(CommandOptions::ShowEnds),
            cmd if cmd == "-n" || cmd == "--number" => Ok(CommandOptions::Number),
            cmd if cmd == "-h" || cmd == "--help" => Ok(CommandOptions::Help),
            cmd if cmd == "-T" || cmd == "--show-tabs" => Ok(CommandOptions::ShowTabs),
            cmd if cmd == "-b" || cmd == "--number-nonblank" => Ok(CommandOptions::NumberNonBlank),
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
        println!("Usage: cat [OPTION]... [FILE]...");
        println!("Concatenate FILE(s), or standard input, to standard output.\n");
        println!("With no FILE, or when FILE is -, read standard input\n");

        let commands = ["ShowEnds", "Number", "Help"];

        fn make_command_string(command: &str, description: &str) -> String {
            format!("{command}\t\t{description}")
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
                &_ => unreachable!("Please implement command: '{command}' or remove it"),
            }
        }

        exit(1)
    }

    fn weight(&self) -> u32 {
        match self {
            CommandOptions::NumberNonBlank => 0,
            CommandOptions::ShowEnds => 1,
            CommandOptions::ShowTabs => 2,
            CommandOptions::Number => 3,
            CommandOptions::Help => 4,
        }
    }
}

fn read_args() -> (Vec<CommandOptions>, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let mut options: HashSet<CommandOptions> = HashSet::new();
    let mut file_paths: Vec<String> = vec![];
    let mut i = 1;

    while args[i].starts_with('-') {
        match CommandOptions::try_from(args[i].as_str()) {
            Ok(c) => {
                match c {
                    CommandOptions::Help => CommandOptions::help(),
                    _ => {
                        options.insert(c);
                    }
                };
            }
            Err(_) => (),
        };

        i += 1;
    }

    let mut options = Vec::from_iter(options);

    options.sort_by(|x, y| x.weight().cmp(&y.weight()));

    for j in i..args.len() {
        file_paths.push(args[j].clone());
    }

    return (options, file_paths);
}

fn display_args(options: &Vec<CommandOptions>, file_paths: &Vec<String>) -> () {
    println!("Options: {:?}, File paths: {:?}", options, file_paths)
}

fn concat_strings_from_files(file_paths: &Vec<String>) -> String {
    let mut res = String::new();

    for path in file_paths {
        let str = fs::read_to_string(path);
        match str {
            Ok(s) => res.push_str(&s),
            Err(e) => {
                eprintln!("ERROR: Couldn't read file: {e}");
                exit(1);
            }
        }
    }

    res
}

fn process_string(mut str: String, options: &Vec<CommandOptions>) -> String {
    for option in options {
        match option {
            CommandOptions::Number => {
                if options.contains(&CommandOptions::NumberNonBlank) {
                    continue;
                };
                str = str
                    .lines()
                    .enumerate()
                    .map(|(i, s)| format!("{index} {s}\n", index = i + 1))
                    .collect();
            }
            CommandOptions::ShowEnds => str = str.lines().map(|s| format!("{s}$\n")).collect(),
            CommandOptions::ShowTabs => str = str.replace("\t", "^I"),
            CommandOptions::NumberNonBlank => {
                let mut i = 0;
                str = str
                    .lines()
                    .map(|s| {
                        let index = if s.is_empty() == true {
                            format!("{:len$}", "", len = i)
                        } else {
                            i += 1;
                            i.to_string()
                        };
                        format!("{index} {s}\n")
                    })
                    .collect();
            },
            CommandOptions::Help => continue
        };
    }
    str
}

fn main() {
    let (options, file_paths) = read_args();
    let concat_str = concat_strings_from_files(&file_paths);
    let concat_str = process_string(concat_str, &options);
    println!("{concat_str}");
    display_args(&options, &file_paths);
}
