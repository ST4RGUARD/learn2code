// simple area function
// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }
//
// fn main() {
//     let length = 6;
//     let width = 6;
//     println!("The area of the rectangle is",
//              area(length, width)
//              );
// }
//
// area with struct
//
// struct Rectangle {
//     width: u32,
//     length: u32,
// }
//
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.length
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 6,
//         length: 6,
//     };
//
//     println!("The area of the rectangle is {}", area(&rect1));
// }
//
// area with struct impl
//
// struct Rectangle {
//     width: u32,
//     length: u32,
// }
//
// impl Rectangle {
//     pub fn area(length: u32, width: u32) -> u32 {
//         length * width
//     }
// }
//
// fn main() {
//     // calling this rect1 is not acurate we can call area but that should be var name also
//     difining  the rect should be rect and then we can call all functions from it later like area
//     see ex below
//     let rect1 = Rectangle::area(5, 5);
//     println!("The area of the rectangle is {}", rect1);
// }
//
// ai enhanced
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 5,
        length: 5,
    };
    println!("The area of the rectangle is {}", rect1.area());
}
