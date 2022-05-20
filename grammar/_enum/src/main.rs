enum IpAddrKind {
    V4,
    V6,
}

// 不需要使用额外的 struct
// 每个变体可以拥有不同的类型和数据
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32, z: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 使用 impl 定义方法
impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V4);

    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let a = Message::Quit;
    let b = Message::Move { x: 1, y: 1, z: 3 };
    let c = Message::Write(String::from("::"));
    let b = Message::ChangeColor(255, 255, 255);
    b.call();
}

fn route(ip_kind: IpAddrKind) {}
