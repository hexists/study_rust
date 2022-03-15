fn main() {
	let number = 6; 

	if number % 4 == 0 {
		println!("4로 나눠 떨어짐");
	} else if number % 3 == 0 {
		println!("3으로 나눠 떨어짐");
	} else {
		println!("4,3으로 나눠 떨어지지 않음");
	}

	let condition = true;
	let number = if condition {
		5
	} else {
		6
	};

	println!("number의 값: {}", number);

	let mut n = 0;
	loop {
		if n > 5 {
			break;
		}

		println!("{} in loop", n);
		n += 1;
	}

	n = 0;

	while n <= 5 {
		println!("{} in while", n);
		n += 1;
	}

	let  a = [10, 20, 30, 40, 50];

	for ele in a.iter() {
		println!("ele: {}", ele);
	}
	for (idx, ele) in a.iter().enumerate() {
		println!("ele[{}] = {}", idx, ele);
	}
}
