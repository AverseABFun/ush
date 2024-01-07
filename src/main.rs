use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Child;
use std::fs;
extern crate clap;
use clap::Parser;

fn list_dir() {
    let paths = fs::read_dir(env::current_dir().unwrap()).unwrap();

    println!("Contents of \"{}\"", env::current_dir().unwrap().into_os_string().into_string().unwrap()
    .replace("\\","/").replace("C:/","/").replace("/ush-sandbox",""));

    for path in paths {
        println!("{}", path.unwrap().path().to_str().unwrap().replace("\\","/").split("/").last().unwrap())
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'q', long = "quiet")]
    quiet: bool
}
fn main() {
    let args = Cli::parse();
    if !args.quiet {
        print!("Welcome to ush, the Update SHell\n");
        print!("This is designed for use in changing hosted files, and originally created for use in making a collaboratively created ARG\n");
    }
    loop {
        print!("{}> ",env::current_dir().unwrap().into_os_string().into_string().unwrap()
                                        .replace("\\","/").replace("C:/","/").replace("/ush-sandbox",""));
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command: Option<Child> = None;

        while let Some(command) = commands.next() {
            
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    
                    let new_dir = &(if !args.clone().peekable().peek().map_or("/", |x| *x).starts_with("/") {
                        "/ush-sandbox/".to_owned()+args.peekable().peek().map_or("/", |x| *x)
                    } else {
                        "/ush-sandbox".to_owned()+args.peekable().peek().map_or("/", |x| *x)
                    });
                    if new_dir.replace("\\","/").split("/").nth(1).unwrap() != "ush-sandbox" {
                        break;
                    }
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "edit" => {
                    print!("This has not been made yet, soooooooo");
                    previous_command = None;
                }
                "exit" => return,
                "ls" => {
                    list_dir();
                    previous_command = None;
                }
                "dir" => {
                    list_dir();
                    previous_command = None;
                }
                "mkdir" => {
                    let new_dir = &(if !args.clone().peekable().peek().map_or("/", |x| *x).starts_with("/") {
                        "/ush-sandbox/".to_owned()+args.peekable().peek().map_or("/", |x| *x)
                    } else {
                        "/ush-sandbox".to_owned()+args.peekable().peek().map_or("/", |x| *x)
                    });
                    if new_dir.replace("\\","/").split("/").nth(1).unwrap() != "ush-sandbox" {
                        break;
                    }
                    let root = Path::new(new_dir);
                    if let Err(e) = fs::create_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "la" => {
                    list_dir();
                    previous_command = None;
                },
                &_ => todo!()
            }
        }

        if let Some(ref mut final_command) = previous_command {
            
            final_command.wait().unwrap();
        }
    }
}
