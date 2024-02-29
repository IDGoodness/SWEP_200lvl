use std::io;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

fn main() {
    // let n = 10; // The value, 10 can be changed to a preferred nth term of the Fibonacci series


    
    // Take input from user this is n

    println!("Enter a value (where the Fibonacci series will stop): ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Invalid input");

    for i in 0..n {
        println!("Fibonacci({}) = {}", i, fibonacci(i));
    }
}