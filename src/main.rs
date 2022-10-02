use std::env;
mod commands;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut command: char = '0';
    let mut values: Vec<&str> = Vec::new();
    args.remove(0);
    for argument in &args {
        let arg = argument.as_str();
        let first_char: char = argument.chars().nth(0).unwrap();
        match &first_char {
            '-' => match argument.as_str() {
                "-e" | "--eval" => {
                    command = '1';
                }

                "-p" | "--print" => {
                    command = '2';
                }

                "-v" | "--version" => {
                    command = '3';
                }

                "-h" | "--help" => {
                    command = '4';
                }

                "-c" | "--compile" | "-b" | "--build" => {
                    command = '5';
                }

                _ => {
                    println!("unknown argument {argument}!");
                    break;
                }
            },
            _ => {
                values.push(arg);
                if command == '0' {
                    command = '5';
                }
            }
        }
    }

    match &command {
        '1' => {
            commands::eval(values[0].to_string());
        }

        '2' => {
            commands::print(values[0].to_string());
        }

        '3' => {
            commands::version();
        }

        '4' => {
            commands::help();
        }

        '5' => {
            commands::build(values[0].to_string(), values[1].to_string());
        }

        _ => {
            println!("lavash --help for all commands");
        }
    }
}
