use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let f: u32 = File::open("hello.txt");
    
    let f = File::open("hello.txt");

    /*
    let f = match f {
        Ok(file) => file, 
        Err(error) => {
            panic!("파일 열기 실패: {:?}", error);
        }
    };
    */

    let f = match f {
        Ok(file) => file, 
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일을 생성하지 못했습니다: {:?}", e),
            },
            other_error => panic!("파일을 열지 못했습니다: {:?}", other_error),
        },
    };
}
