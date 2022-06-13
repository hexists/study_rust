use std::env;
use std::process;
use std::fs;

/*
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });

    println!("검색어   : {}", config.query);
    println!("대상 파일: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);

        process::exit(1);
    }
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("검색어   : {}", config.query);
    println!("대상 파일: {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("파일을 읽지 못했습니다.");

    println!("파일 내용:\n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

/* STEP3
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("검색어   : {}", config.query);
    println!("대상 파일: {}", config.filename);


    let contents = fs::read_to_string(config.filename).expect("파일을 읽지 못했습니다.");

    println!("파일 내용:\n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
*/

/* STEP2
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let (query, filename) = parse_config(&args);

    let contents = fs::read_to_string(filename).expect("파일을 읽지 못했습니다.");

    println!("파일 내용:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
*/

/* STEP1
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("검색어   : {}", query);
    println!("대상 파일: {}", filename);

    let contents = fs::read_to_string(filename).expect("파일을 읽지 못했습니다.");

    println!("파일 내용:\n{}", contents);
}
*/
