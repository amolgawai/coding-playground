#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.height < self.height && other.width < self.width
    }

    // class (static) method
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let a_rect = Rectangle {
        width: 10,
        height: 15,
    };
    println!("A Rectangle - {:?}, area - {}", a_rect, a_rect.area());

    let a_square = Rectangle::square(9);
    println!(
        "Does {:?} fit inside {:?} -> {}",
        a_square,
        a_rect,
        a_rect.can_hold(&a_square)
    );
}
