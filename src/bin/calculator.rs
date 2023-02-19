use std::error::Error;
use std::io::{self, BufRead, Write};
use std::{fmt, process};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stack: Vec<f64> = Vec::new();
    print_cmd_head();
    for line_result in io::stdin().lock().lines() {
        let line =
            line_result.map_err(|err| ParserError(format!("Failed to read stdin: {err}")))?;

        match parse_line(line) {
            Ok(token) => handle_token(&mut stack, token),
            Err(err) => println!("{err}"),
        };

        print_cmd_head();
    }
    Ok(())
}

fn parse_line(line: String) -> Result<Token, ParserError> {
    match line.as_str() {
        "quit" | "q" => Ok(Token::Quit),
        "dump" | "d" => Ok(Token::Dump),
        "+" => Ok(Token::Add),
        "-" => Ok(Token::Subtract),
        "*" => Ok(Token::Multiply),
        "/" => Ok(Token::Divide),
        _ => match line.parse() {
            Ok(number) => Ok(Token::Number(number)),
            Err(_) => Err(ParserError(format!(
                "Unknown command or invalid float: {line}"
            ))),
        },
    }
}

fn handle_token(stack: &mut Vec<f64>, token: Token) {
    let result = match token {
        Token::Quit => {
            println!("K, bye");
            process::exit(0);
        }
        Token::Dump => {
            dump_stack(&stack);
            Ok(())
        }
        Token::Number(number) => Ok(stack.push(number)),
        Token::Add => {
            // I miss inheritance, maybe traits will help
            apply_binary_operator(stack, |a, b| Ok(a + b))
        }
        Token::Subtract => {
            apply_binary_operator(stack, |a, b| Ok(a - b))
        }
        Token::Multiply => {
            apply_binary_operator(stack, |a, b| Ok(a * b))
        }
        Token::Divide => {
            // TODO handle divide by zero
            apply_binary_operator(stack, |a, b| Ok(a / b))
        }
    };
    if let Err(err) = result {
        println!("{err}");
    }
}

fn apply_binary_operator<F: Fn(f64, f64) -> Result<f64, CalculatorError>>(
    stack: &mut Vec<f64>,
    calculate: F,
) -> Result<(), CalculatorError> {
    match stack.len() {
        0 => Err(CalculatorError("The stack is empty!".to_string())),
        1 => Err(CalculatorError("Only 1 number on the stack!".to_string())),
        _ => {
            let number2 = stack.pop().expect("Got at least 2 numbers");
            let number1 = stack.pop().expect("Still got at least 1 number");
            stack.push(calculate(number1, number2)?);
            dump_stack(&stack);
            Ok(())
        }
    }
}

fn print_cmd_head() -> () {
    print!("> ");
    io::stdout()
        .flush()
        .expect("Don't bother handling as print! probably doesn't either");
}

fn dump_stack(stack: &Vec<f64>) -> () {
    for number in stack {
        println!("{number}");
    }
}

enum Token {
    Quit,
    Dump,
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(f64),
}

#[derive(Debug)]
struct ParserError(String);
impl Error for ParserError {}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Debug)]
struct CalculatorError(String);
impl Error for CalculatorError {}
impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}
