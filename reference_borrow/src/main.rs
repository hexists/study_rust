fn main() {
	let s1 = String::from("hello");
	let len = calc_length(&s1);

	println!("'{}'의 길이는 {}입니다.", s1, len);

	let mut s2 = String::from("hello");
	println!("before s2: {}", s2);
	change(&mut s2);
	println!("after  s2: {}", s2);
}

fn calc_length(s: &String) -> usize {
	s.len()
}

fn change(some_string: &mut String) {
	some_string.push_str(", world");
}
