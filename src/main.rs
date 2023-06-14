use std::cmp::Ordering;
use std::collections::HashSet;
use std::{env, fs, process::exit};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
enum CommandOptions {
    NumberNonBlank,
    ShowEnds,
    Number,
    ShowTabs,
    Help,
}

enum CommandPriority {
    LOW,
    MIDDLE,
    HIGH,
}

struct CommandStruct {
    name: String,
    commands: Vec<String>,
    description: String,
    priority: CommandPriority,
}

impl CommandPriority {
    fn value(&self) -> u8 {
        match self {
            CommandPriority::LOW => 0,
            CommandPriority::MIDDLE => 1,
            CommandPriority::HIGH => 2,
        }
    }
}

impl CommandOptions {
    fn value(&self) -> CommandStruct {
        let sf = String::from;
        match self {
            CommandOptions::Help => CommandStruct {
                name: sf("Help"),
                commands: vec![sf("-h"), sf("--help")],
                description: sf("display this help and exit"),
                priority: CommandPriority::LOW,
            },
            CommandOptions::ShowEnds => CommandStruct {
                name: sf("ShowEnds"),
                commands: vec![sf("-E"), sf("--show-ends")],
                description: sf("display $ at end of each line"),
                priority: CommandPriority::LOW,
            },
            CommandOptions::ShowTabs => CommandStruct {
                name: sf("ShowTabs"),
                commands: vec![sf("-T"), sf("--show-tabs")],
                description: sf("display TAB characters as ^I"),
                priority: CommandPriority::LOW,
            },
            CommandOptions::Number => CommandStruct {
                name: sf("Number"),
                commands: vec![sf("-n"), sf("--number")],
                description: sf("number all output lines"),
                priority: CommandPriority::MIDDLE,
            },
            CommandOptions::NumberNonBlank => CommandStruct {
                name: sf("NumberNonBlank"),
                commands: vec![sf("-b"), sf("--number-nonblank")],
                description: sf("number nonblank output lines"),
                priority: CommandPriority::HIGH,
            },
        }
    }

    fn help() -> () {
        println!("Usage: cat [OPTION]... [FILE]...");
        println!("Concatenate FILE(s), or standard input, to standard output.\n");
        println!("With no FILE, or when FILE is -, read standard input\n");

        let max_len =
            CommandOptions::iter().fold(0, |acc, option| match option.value().commands.last() {
                Some(s) => match s.len().cmp(&acc) {
                    Ordering::Greater => s.len(),
                    _ => acc,
                },
                None => {
                    eprintln!(
                        "ERROR: Theres no commands for {}. Please provide at least 1 command",
                        option.value().name
                    );
                    exit(1)
                }
            });

        let default_space = 10;

        for option in CommandOptions::iter() {
            let option_val = option.value();
            let space = (max_len + default_space) - option_val.commands.last().unwrap().len();
            println!(
                "  {command}{:space$}{description}",
                "",
                space = space,
                command = option_val.commands.join(", "),
                description = option_val.description
            )
        }

        exit(1)
    }

    fn priority(&self) -> u8 {
        self.value().priority.value()
    }
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


fn read_args() -> (Vec<CommandOptions>, Vec<String>) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("ERROR: You should specify at least path to a file");
        exit(1)
    }

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

    options.sort_by(|x, y| x.priority().cmp(&y.priority()));

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
            }
            CommandOptions::Help => continue,
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
