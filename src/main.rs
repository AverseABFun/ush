use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio, ChildStdin};
extern crate clap;
use clap::Parser;

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
                                        .replace("\\","/").replace("C:/","/").replace("/ush-sandbox","/"));
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
                        "/".to_owned()+args.peekable().peek().map_or("/", |x| *x)
                    } else {
                        args.peekable().peek().map_or("/", |x| *x).to_string()
                    });
                    print!("{}", new_dir);
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
                command => {
                    let stdin: Stdio = (&previous_command.unwrap()).as_ref().stdin.map_or(Stdio::inherit(), |output: ChildStdin| {
                        Stdio::from(output)
                    });

                    let stdout = if commands.peek().is_some() {
                        
                        
                        Stdio::piped()
                    } else {
                        
                        
                        Stdio::inherit()
                    };
                    if command.clone().starts_with("bash") || command.clone().starts_with("zsh") || command.clone().starts_with("dash") || command.clone().starts_with("cmd") || command.clone().ends_with("bash") || command.clone().ends_with("zsh") || command.clone().ends_with("dash") || command.clone().ends_with("cmd") {
                        print!("haha, you THOUGHT");
                        break;
                    }
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(ref mut final_command) = previous_command {
            
            final_command.wait().unwrap();
        }
    }
}
