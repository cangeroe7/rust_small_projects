fn main() {
    println!("Hello, world!");
}



















// use std::fs::Files;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username =  String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn main() {
    
// }





// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }


// fn value(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }


// fn main() {
//     let penny = Coin::Penny;
//     println!("lucky penny value: {:?}", value(penny));
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width <= other.width && self.height <= other.height
//     }

//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 20,
//         height: 50
//     };
//     let rect2 = Rectangle {
//         width: 25,
//         height: 60
//     };

//     let square1 = Rectangle::square(20);

//     println!("rect1 area: {}", rect1.area());
//     println!("rect1 fits in rect2: {}", rect1.can_hold(&rect2));
//     println!("square1 area: {}", square1.area());

// }