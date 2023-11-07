#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Write(String),
    Quit,
    Move {x: i32, y: i32},
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(x) =>  println!("This is the message: {}", x),
            (_) => println!("wild card here!"),
        }
       
    }
}

fn main() {
    println!("Hello, world!");
    println!("The enum looks like this: {:#?}", IpAddressKind::V4);
    println!("The enum looks like this: {:#?}", IpAddress::V6(String::from("::1")));
    let m = Message::Write(String::from("Hello there Enum"));
    println!("The message looks like this: {:#?}", m.call());

}

