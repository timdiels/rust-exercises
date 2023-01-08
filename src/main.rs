use std::fmt;
use std::io;
use std::io::Write;
use std::ops::RangeInclusive;

const PRIMES_BELOW_1000: [usize; 168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

fn main() {
    assert_eq!(get_perfect_numbers(), [6, 28, 496]);
    assert_eq!(get_primes(), PRIMES_BELOW_1000);

    print_quadratic_roots(1.0, -3.0, 2.0); // 1 and 2
    print_quadratic_roots(4.0, 2.0, 2.0); // complex
    print_quadratic_roots(2.0, 2.0, 0.5); // -0.5

    {
        let mut items: Vec<i32> = vec![];
        inplace_sort(&mut items);

        let mut items = vec![3, 1, 8, 1, 0, 5];
        inplace_sort(&mut items);
        assert_eq!(items, [0, 1, 1, 3, 5, 8]);

        let mut items = vec!["d", "b", "f", "b", "a", "c"];
        inplace_sort(&mut items);
        assert_eq!(items, ["a", "b", "b", "c", "d", "f"]);
    }

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
    for divisor in divisors.iter() {
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

// Exercise: Write a program that prints the roots of a quadratic equation (given hardcoded
// coefficients a, b and c).
/// a x^2 + b x + c
fn print_quadratic_roots(a: f64, b: f64, c: f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    // is there a nicer way to say np.close?
    if discriminant.abs() < 1e-10 {
        let root = -b / (2.0 * a);
        println!("Both roots are: {root}");
    } else if discriminant > 0.0 {
        let roots = [-1.0, 1.0].map(|sign| (-b + sign * discriminant.sqrt()) / (2.0 * a));
        println!("First root: {}, second root {}", roots[0], roots[1]);
    } else {
        println!("I'll skip on complex numbers");
    }
}

// Exercise: Implement the Sieve of Eratosthenes to find all prime numbers up to 1000
fn get_primes() -> Vec<usize> {
    const RANGE: RangeInclusive<usize> = 2..=1000;
    let mut is_prime = [true; *RANGE.end() - 1];

    // Mark non-primes
    for (i, prime) in RANGE.enumerate() {
        if is_prime[i] {
            for factor in 2..=(RANGE.end() / prime) {
                is_prime[prime * factor - *RANGE.start()] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, is_prime)| match *is_prime {
            true => Some(i + *RANGE.start()),
            false => None,
        })
        .collect()
}

// Exercise: Write a program that sorts a vector of integers/strings “in place” using a
// handwritten selection sort: find the smallest number, move it to the front, then find the
// smallest of the rest, move it to second place etc.
fn inplace_sort<T: Ord + fmt::Debug>(items: &mut Vec<T>) {
    if items.is_empty() {
        return;
    }

    for i in 0..(items.len() - 1) {
        let (index_of_min, _) = items
            .iter()
            // TODO placing skip before enumerate doesn't work, seems to not skip then, why?
            .enumerate()
            .skip(i)
            .min_by(|(_, item1), (_, item2)| (item1).cmp(item2))
            .unwrap();
        // Seems mem::swap on the items is impossible as it would require 2 iter_mut => compile
        // error "second mutable borrow"
        items.swap(i, index_of_min);
    }
}
