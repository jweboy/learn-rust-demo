// fn声明
// 小写 下划线分隔
fn main() {
    println!("Hello, world!");
    another(2, 5); // arguments

    let y = {
        // y这个代码块是一个表达式
        let x = 1;
        // x + 3 // 表达式
        x + 3; // 加上分号就是一个语句，没有返回值，实际语句返回的就是 ()
    };

    println!("{:?}", y);
}

// 签名里必须指明每个参数类型
fn another(x: i32, y: i32) -> i32 {
    // parameter
    x + y;
    3
}
