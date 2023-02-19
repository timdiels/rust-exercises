use std::io;
use std::io::Write;

fn main() {
    assert_eq!(get_perfect_numbers(), [6, 28, 496]);

    loop {
        print_divisors();
    }
}

fn ask_text() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line
}

fn ask_number() -> u32 {
    loop {
        break match ask_text().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Huh? Please enter a number ");
                continue;
            }
        };
    }
}

// Exercise: Write a rust program that prints all the divisors of a positive integer.
// Print the divisors on one line, separated by a comma without trailing comma using an explicit loop.
fn print_divisors() {
    print!("Please enter number to get divisors of ");
    io::stdout().flush().unwrap();
    let number = ask_number();
    let divisors = get_divisors(number);

    print!("Divisors: ");
    // iter() to avoid moving ownership
    for divisor in &divisors {
        let is_last = *divisor == number; // using peekable would be better
        if is_last {
            println!("{divisor}")
        } else {
            print!("{divisor}, ")
        }
    }

    print!("Divisors using join(): ");
    io::stdout()
        .write(
            divisors
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(", ")
                .as_bytes(),
        )
        .unwrap();
    println!();
}

/// Get divisors in ascending order
fn get_divisors(number: u32) -> Vec<u32> {
    let mut divisors: Vec<u32> = Vec::new();
    for i in 1..=number {
        if number % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

// Exercise: Write a function is_perfect(n) which returns true if the number is “perfect”. Find all
// perfect numbers below 1000.
fn get_perfect_numbers() -> Vec<u32> {
    (1..1000).filter(|num| is_perfect(*num)).collect()
}

fn is_perfect(number: u32) -> bool {
    let mut divisors = get_divisors(number);
    divisors.pop();
    let sum: u32 = divisors.iter().copied().sum();
    sum == number
}
