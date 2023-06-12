use std::{env, fs, process::exit};

#[derive(Debug)]
enum CommandOptions {
    ShowEnds,
    Number
}

impl TryFrom<&str> for CommandOptions {
    type Error = ();

    fn try_from(cmd: &str) -> Result<Self, Self::Error> {
        match cmd {
            cmd if cmd == "-E" || cmd == "--show-ends" => Ok(CommandOptions::ShowEnds),
            cmd if cmd == "-n" || cmd == "--number" => Ok(CommandOptions::Number),
            &_ => {
                eprintln!("ERROR: Couldn't find command: {cmd}");
                exit(1);
            }
        }
    }
}

fn read_args() -> (Vec<CommandOptions>, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let mut options: Vec<CommandOptions> = vec![];
    let mut file_paths: Vec<String> = vec![];

    for i in 1..args.len() {
        let arg = args[i].clone();
        if arg.starts_with("-") == false {
            file_paths.push(arg);
        } else {
            let cmd_option = CommandOptions::try_from(arg.as_str());
            match cmd_option {
                Ok(c) => options.push(c),
                Err(_) => ()
            };
        }
    }

    return (options, file_paths);
}

fn display_args(options: &Vec<CommandOptions>, file_paths: &Vec<String>) -> () {
    println!("Options: {:?}, File paths: {:?}", options, file_paths)
}

fn main() {
    let (options, file_paths) = read_args();
    display_args(&options, &file_paths);
}
