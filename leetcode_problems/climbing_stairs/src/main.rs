fn climb_stairs(n: i32) -> i32 {
    let mut fib = (0, 1);

    for _ in 0..n {
        fib = (fib.1, fib.0 + fib.1);
    }
    fib.1
}

fn main() {
    let n = 45;
    let steps = climb_stairs(n);
    println!("Amount of steps: {}", steps);
}