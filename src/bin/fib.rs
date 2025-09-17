fn main() {
    print!("{}", fib(4));
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;   
    }
    fib(n - 1) + fib(n - 2)
}