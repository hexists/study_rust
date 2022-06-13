use std::thread;


fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("벡터: {:?}", v);
    });

    handle.join().unwrap();
}
