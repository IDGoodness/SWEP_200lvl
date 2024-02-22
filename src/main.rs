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
    let n = 10; // Change this value to generate a different number of Fibonacci numbers

    for i in 0..n {
        println!("Fibonacci({}) = {}", i, fibonacci(i));
    }
}
