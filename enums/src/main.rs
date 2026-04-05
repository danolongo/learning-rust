struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method
    }
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

fn main() {
    let m = Message::Write(String::from("hi"));
    m.call();

    let some_num = Some(5);
    let some_char = Some('c');
    let abs_num: Option<i32> = None;

    let sum = 10 + some_num.unwrap();
    println!("{}", sum);
}
