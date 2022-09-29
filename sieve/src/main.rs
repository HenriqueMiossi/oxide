use std::io;
use std::io::Write;
use std::collections::LinkedList;

fn main() {
    print!("Insert a number: ");
    io::stdout().flush().unwrap();

    let number: i32;
    let mut number_str: String = String::new();

    io::stdin().read_line(&mut number_str)
        .expect("Didn't recieved any number");

    number = number_str.trim().parse()
        .expect("Input is not a number");

    is_prime(number);
}

fn is_prime(number: i32) {
    let mut is_prime = true;
    let mut list = LinkedList::new();

    // Treats 1, 0 and negative numbers, which cannot be prime
    if number <= 1 {
        println!("The number {number} is not prime!");
        return;
    }

    for i in 2..number {
        if number % i == 0 {
            is_prime = false;
            list.push_back(i);
        }
    }

    if is_prime {
        println!("The number {number} is prime!");
        return;
    } else {
        println!("The number {number} is not prime!");
        println!("The number is divisible by: {:?}", list);
        return;
    }
}
