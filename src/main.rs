use std::io;

fn main() {

    println!("--- Welcome to the Fibonacci number generator --- \n");

    loop {
        let mut input = String::new();

        println!(" please enter a positive integer or type \"quit\" to exit the program");

        // get user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "quit" {
            break;
        }

        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Please provide an unassigned integer"),
        };

        // find the corresponding fibonacci number & give feedback to the user
        println!(
            "The {}th Fibonacci number is {}",
            input,
            nth_fibonacci(input)
        );
    }
}

fn nth_fibonacci(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 0;
    let mut count = 0;

    while count < n {
        let temp = a + b;
        b = a;
        a = temp;
        count += 1;
    }

    b
}
