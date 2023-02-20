use std::io::{self, BufRead, Write};
use std::iter::Iterator;
use std::process;

#[macro_use]
extern crate is_close;

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
    println!("Ctrl-d? Trying to be fancy huh?");
}

fn parse_line(line: String) -> Result<Command, String> {
    match line.trim() {
        "quit" | "q" => Ok(Command::Quit),
        "dump" | "d" => Ok(Command::Dump),
        "+" => Ok(Command::PerformBinaryCalculation(Box::new(Add))),
        "-" => Ok(Command::PerformBinaryCalculation(Box::new(Subtract))),
        "*" => Ok(Command::PerformBinaryCalculation(Box::new(Multiply))),
        "/" => Ok(Command::PerformBinaryCalculation(Box::new(Divide))),
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
        Command::PerformBinaryCalculation(calculation) => calculation.apply_to_stack(stack),
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
    PushNumber(f64),
    PerformBinaryCalculation(Box<dyn BinaryCalculation>),
}

trait BinaryCalculation {
    fn calculate(&self, number1: f64, number2: f64) -> Result<f64, String>;

    fn apply_to_stack(&self, stack: &mut Vec<f64>) -> Result<(), String> {
        match stack.len() {
            0 => Err("The stack is empty!".to_string()),
            1 => Err("Only 1 number on the stack!".to_string()),
            _ => {
                let numbers = &stack[stack.len() - 2..];
                let number = self.calculate(numbers[0], numbers[1])?;
                stack.truncate(stack.len() - 2);
                stack.push(number);
                print_stack(&stack);
                Ok(())
            }
        }
    }
}

struct Add;
impl BinaryCalculation for Add {
    fn calculate(&self, number1: f64, number2: f64) -> Result<f64, String> {
        Ok(number1 + number2)
    }
}

struct Subtract;
impl BinaryCalculation for Subtract {
    fn calculate(&self, number1: f64, number2: f64) -> Result<f64, String> {
        Ok(number1 - number2)
    }
}

struct Multiply;
impl BinaryCalculation for Multiply {
    fn calculate(&self, number1: f64, number2: f64) -> Result<f64, String> {
        Ok(number1 * number2)
    }
}

struct Divide;
impl BinaryCalculation for Divide {
    fn calculate(&self, number1: f64, number2: f64) -> Result<f64, String> {
        if is_close!(number2, 0.0) {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(number1 / number2)
        }
    }
}
