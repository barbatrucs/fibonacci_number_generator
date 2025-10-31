use std::io;

fn main() {
    // The Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21...

    let mut input = String::new();

    println!("--- Welcome to the Fibonacci number generator --- \n");
    println!(" please enter a positive integer");

    // get user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
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

fn nth_fibonacci(n: u32) -> u32 {
    // base case : if n = 0 or n = 1 then return n
    if n <= 1 {
        return n;
    }

    // recursive case : sum of the 2 preceding Fibonacci numbers
    return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
}
