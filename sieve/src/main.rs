use std::io;
use std::io::Write;

fn main() {
    print!("Insert a number: ");
    io::stdout().flush().unwrap();

    let number: i32;
    let mut number_str: String = String::new();

    io::stdin().read_line(&mut number_str)
        .expect("Didn't recieved any number");

    number = number_str.trim().parse()
        .expect("Input is not a number");

    let (is_prime, list) = is_prime(number);

    if is_prime {
        println!("The number is prime!");
    } else {
        println!("The number is not prime!");
        print!("It is divisible by: ");
        io::stdout().flush().unwrap();

        for i in 0..list.len() {
            print!("{}, ", list[i as usize]);
            io::stdout().flush().unwrap();
        }

        println!("end");
    }
}

fn is_prime(number: i32) -> (bool, Vec<i32>) {
    let mut is_prime = true;
    let mut list = Vec::new();

    // Treats 1, 0 and negative numbers, which cannot be prime
    if number <= 1 {
        return (false, list);
    }

    for i in 2..number {
        if number % i == 0 {
            is_prime = false;
            list.push(i);
        }
    }

    if is_prime {
        return (true, list);
    } else {
        return (false, list);
    }
}
