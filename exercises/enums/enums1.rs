use std::fmt::Debug;
// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
