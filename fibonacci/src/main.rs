fn main() {
    println!("Fibonacci");

	let mut n = 100;  // thread 'main' panicked at 'attempt to add with overflow', src/main.rs:17:9
	n = 10;

	let fibo_n = fibo(n);

	println!("fibo_n: {}", fibo_n);
}

fn fibo(n: i32) -> i32 {
	if n == 0 {
		0  // err
	} else if n == 1 || n == 2 {
		1
	} else {
		fibo(n - 2) + fibo(n - 1)
	}
}
