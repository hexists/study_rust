#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1: {:?}", rect1);
    println!("rect1: {:#?}", rect1);

    println!(
        "사각형의 면적: {} 제곱 픽셀",
        area(&rect1)
    );

    let rect2 = Rectangle { width: 10, height: 40 };

    println!(
        "사각형의 면적: {} 제곱 픽셀",
        rect2.area()
    );

    let rect3 = Rectangle { width: 60, height: 45 };

    println!("\nrect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));

    Rectangle::square(3);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 구조체 인스턴스를 받지 않는 함수
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
