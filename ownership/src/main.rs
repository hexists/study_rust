fn main() {
	/* 
	s2 = s1 선언시, 값이 move됨
	let s1 = String::from("hello");
	let s2 = s1;

	println!("{}, world!", s1);
	println!("{}, world!", s2);
	*/

	let s1 = String::from("hello");
	let s2 = s1.clone();

	println!("s1 = {}, s2 = {}", s1, s2);

	// ex-4-3
	let s = String::from("hello");

	takes_ownership(s);  // s값 함수 내로 이동, s는 invalid

	let x = 5;

	makes_copy(x);  // x 함수로 이동, 하지만 i32 타입은 복사 수행하므로 유효

	// test copy, str type
	let a = "Hello";
	let b = a;

	println!("a = {}, b = {}", a, b);
}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}
