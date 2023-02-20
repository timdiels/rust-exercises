use std::io::{self, BufRead, Write};
use std::iter::Iterator;
use std::process;

fn main() {
    let mut stack: Vec<f64> = Vec::new();
    let command_results = io::stdin().lock().lines().map(|line_result| {
        parse_line(line_result.expect("Failed to read stdin"))
            .and_then(|command| handle_command(&mut stack, command))
    });

    print_cli_head();
    for result in command_results {
        if let Err(err) = result {
            println!("{err}");
        }
        print_cli_head();
    }
}

fn parse_line(line: String) -> Result<Command, String> {
    match line.as_str() {
        "quit" | "q" => Ok(Command::Quit),
        "dump" | "d" => Ok(Command::Dump),
        "+" => Ok(Command::Add),
        "-" => Ok(Command::Subtract),
        "*" => Ok(Command::Multiply),
        "/" => Ok(Command::Divide),
        _ => match line.parse() {
            Ok(number) => Ok(Command::PushNumber(number)),
            Err(_) => Err(format!("Unknown command or invalid float: {line}")),
        },
    }
}

fn handle_command(stack: &mut Vec<f64>, command: Command) -> Result<(), String> {
    match command {
        Command::Quit => {
            println!("K, bye");
            process::exit(0);
        }
        Command::Dump => {
            print_stack(&stack);
            Ok(())
        }
        Command::PushNumber(number) => Ok(stack.push(number)),
        Command::Add => {
            // I miss inheritance, maybe traits will help
            apply_binary_operator(stack, |a, b| Ok(a + b))
        }
        Command::Subtract => apply_binary_operator(stack, |a, b| Ok(a - b)),
        Command::Multiply => apply_binary_operator(stack, |a, b| Ok(a * b)),
        Command::Divide => {
            // TODO handle divide by zero
            apply_binary_operator(stack, |a, b| Ok(a / b))
        }
    }
}

fn apply_binary_operator<F: Fn(f64, f64) -> Result<f64, String>>(
    stack: &mut Vec<f64>,
    calculate: F,
) -> Result<(), String> {
    match stack.len() {
        0 => Err("The stack is empty!".to_string()),
        1 => Err("Only 1 number on the stack!".to_string()),
        _ => {
            let number2 = stack.pop().expect("Got at least 2 numbers");
            let number1 = stack.pop().expect("Still got at least 1 number");
            stack.push(calculate(number1, number2)?);
            print_stack(&stack);
            Ok(())
        }
    }
}

fn print_cli_head() -> () {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn print_stack(stack: &Vec<f64>) -> () {
    for number in stack {
        println!("{number}");
    }
}

enum Command {
    Quit,
    Dump,
    Add,
    Subtract,
    Multiply,
    Divide,
    PushNumber(f64),
}
