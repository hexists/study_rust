enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("행운의 페니!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state); 
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_number = Some("a string");
    // let absent_number: Option<i32> = None; // error[E0308]: mismatched types ???
    
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six =  plus_one(five);
    let none =  plus_one(None);
}
