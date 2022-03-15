fn main() {
	let mut x = 5;
	println!("x의 값: {}", x);
	x = 6;
	println!("x의 값: {}", x);

	const MAX_POINTS: u32 = 100_000;  // 상수

	let mut spaces = "    ";  // 오류 발생하지 않음
	let spaces = spaces.len();
	println!("spaces의 값: {}", spaces);

	let guess: u32 = "42".parse().expect("숫자가 아닙니다!!");
	println!("gusess의 값: {}", guess);

	let x = 2.0;  // f64, default
	let y: f32 = 2.0;  // f32

	let sum = 5 + 10;
	let difference = 95.5 - 4.3;
	let product = 4 * 30;
	let quotient = 56.7 / 32.2;
	let remainder = 43 % 5;

	let t = true;
	let f: bool = false;

	let c = 'z';
	let imoji = '🤩'; // 4bytes

	// tuple
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z) = tup;

	println!("y의 값: {}", y);
	
	let five_hundred = tup.0;

	// array
	let a = [1, 2, 3, 4, 5];
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	let a = [3; 5];  // [3, 3, 3, 3, 3]

	let first = a[0];
	let element = a[10];
}
