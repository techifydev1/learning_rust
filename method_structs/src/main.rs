fn main() {

    let rect1 = Rectange {
        width: 22,
        height: 44,
    };

    println!("The arean of rect 1 is {}", rect1.area());
}

#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}