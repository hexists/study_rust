fn main() {
	// ref: https://tagilog.tistory.com/376

    println!("Fahrenheit to Celsius");

	let f: f32 = 90.0;
	println!("Fahrenheit: {}", f);

	let c = (f - 32.0) * 5.0 / 9.0;

	let f = (c * 1.8) + 32.0;

	println!("Celsius: {}", c);
	println!("Fahrenheit: {}", f);
}
