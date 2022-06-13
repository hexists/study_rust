use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {  // 스레드 생성, 클로저 전달
        for i in 1..10 {
            println!("새 스레드: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("주 스레드: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();
}
