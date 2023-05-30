// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move(u32, u32, u32),
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("hello")));
    println!("{:?}", Message::Move(1, 2, 3));
    println!("{:?}", Message::ChangeColor);
}
