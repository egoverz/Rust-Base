use std::io::{BufRead, PipeReader, PipeWriter, Write, pipe, stdin};
use std::process::{Command, Stdio};

fn main() {
    loop {
        print!("blazing_shell> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();

        let input: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

        if input.is_empty() {
            continue;
        }

        if input[0] == "exit" {
            break;
        }

        let pipe_index = input.iter().position(|x| *x == "|");


        match pipe_index {
            Some(index) => {

                let (pipe_reader, mut pipe_writer) = pipe().unwrap();

                let first_command = &input[0];
                let first_args = &input[1..index];

                if index + 1 >= input.len() {
                    println!("Ошибка запуска: No command after pipe");
                    continue;
                }

                let second_command = &input[index+1];
                let second_args = &input[index+2..];

                run_command(first_command, first_args, None, Some(pipe_writer));
                run_command(second_command, second_args, Some(pipe_reader), None);


                
            }
            None => {
                let args = &input[1..];
                run_command(&input[0], args, None, None);
            }
        }

    }
}

fn run_command(command: &String, args: &[String], stdin: Option<PipeReader>, stdout: Option<PipeWriter>) {
    let mut child = Command::new(&command);

    child.args(args);


    if stdout.is_some() {
        child.stdout(stdout.unwrap());
    }

    if stdin.is_some() {
        child.stdin(stdin.unwrap());
    }

    let mut running_child = child.spawn();
    let mut running_child = match running_child {
        Ok(child) => child,
        Err(_) => {
            println!("Ошибка запуска: '{}': No such file or directory", command);
            return;
        }
    };

    let status = running_child.wait().unwrap().code().unwrap();

    if status != 0 {
        println!("Exit status: {}", status);
    }
}
