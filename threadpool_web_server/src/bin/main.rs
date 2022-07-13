use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;
use threadpool_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() { // TcpStream 타입의 스트림 반복자를 리턴
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

        /*
        thread::spawn(|| {
            handle_connection(stream);
        });
        */
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // buffer instance 생성

    stream.read(&mut buffer).unwrap(); // 읽은 데이터를 버퍼에 채움

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut contents = fs::read_to_string(filename).unwrap();

    // println!("{}", contents);

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap(); // 문자열을 바이트로 변환
                                                    //
    stream.flush().unwrap(); // 바이트 모두 쓸 때까지 프로그램 실행을 멈추고 기다림
}
