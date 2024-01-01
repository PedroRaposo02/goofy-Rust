#![allow(non_snake_case)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width: width, height: height }
    }

    fn inc_width(&mut self, inc: u32) {
        self.width += inc;
    }
}

fn printRec(rec: &Rectangle) {
    println!("width: {}, height: {}, area: {}", rec.width, rec.height, rec.area());
}

fn main() {              // Program entry point
    let mut rec: Rectangle = Rectangle { width: (5), height: (6) };

    printRec(&mut rec);

    rec.inc_width(4);

    printRec(&mut rec);

    let rec2 = Rectangle::new(3, 5);

    printRec(&rec2)


}