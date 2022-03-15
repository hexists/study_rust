fn main() {
    println!("Hello, world!");

	another_func(5, 3);

	// let x = (let y = 6);  // 구문은 값을 리턴하지 않아 오류 발생

	let y = {
		let x = 3;
		x + 1
	};

	println!("y = {}", y);

	let x = five();

	println!("x = {}", x);
}

fn another_func(x: i32, y: i32) {
	println!("another_func: x = {}, y = {}", x, y);
}

fn five() -> i32 {
	5
}
