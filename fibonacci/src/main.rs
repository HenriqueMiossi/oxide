use std::io;
use std::io::Write;

fn main() {
    print!("Insert the amount of elements of the sequence: ");
    io::stdout().flush().unwrap();

    let quantity: u32;
    let mut quantity_str: String = String::new();

    io::stdin().read_line(&mut quantity_str)
        .expect("Didn't recieved any number");

    quantity = quantity_str.trim().parse().unwrap();

    let vector = fibonacci(quantity);

    for i in 0..vector.len() {
        print!("{} -> ", vector[i as usize]);
        io::stdout().flush().unwrap();
    }

    println!("end");
}

fn fibonacci(quantity: u32) -> Vec<u32> {
    let mut vector: Vec<u32> = Vec::new();

    for i in 0..quantity {
        if i == 0 { 
            vector.push(0);
            continue;
        }
        else if i == 1 { 
            vector.push(1);
            continue;
        };

        let previous_one: u32 = vector[i as usize - 1];
        let previous_two: u32 = vector[i as usize - 2];
        let result: u32 = previous_one + previous_two;

        vector.push(result);
    }

    return vector;
}
