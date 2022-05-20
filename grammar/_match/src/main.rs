#[derive(Debug)]
enum UserState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UserState),
}

fn main() {
    let a = Coin::Quarter(UserState::Alaska);
    println!("{}", value_in_coin(a));

    let b = Some(5);
    let c = plus_one(b);
    let d = plus_one(None);
    println!("{:?}, {:?}", c, d);

    let v = 0u8;

    match v {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        6 => println!("five"),
        // 替代剩余没列出的值
        _ => println!(),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 3),
    }
}

fn value_in_coin(coin: Coin) -> u8 {
    // match匹配必须穷举所有可能
    match coin {
        Coin::Penny => {
            println!("Hello,enny!");
            33
        }
        Coin::Nickel => 2,
        Coin::Dime => 3,
        // 绑定值模式
        Coin::Quarter(state) => {
            println!("Say hello from {:?}", state);
            45
        }
    }
}
