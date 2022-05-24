#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // vector 内存中连续存放 只能存储相同类型
    let v: Vec<i32> = Vec::new();

    // 自动推断出类型了
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mut v = Vec::new();
    v.push('1'); // push之后自动推断出类型
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    // let third: &i32 = &v[30]; // 索引超出会报错

    match v.get(30) {
        // 这里索引超出并不会报错
        Some(x) => println!("{:?}", x),
        None => println!("no element"),
    }

    // 不能在同一作用域内同时拥有可变和不可变引用
    let mut v = vec![1, 2, 3];
    let first = &v[0]; // 不可变借用
                       // v.push(9); // 可变借用
    println!("{:?}", first); // 不可变借用

    // for循环
    let v = vec![1, 2, 3];
    for i in &v {
        println!("iter: {}", i);
    }

    // 遍历修改
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 40;
    }

    for i in v {
        println!("iter: {}", i);
    }

    // 结合 enum
    let rows = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(3.1415926),
    ];
    println!("{:?}", rows);
} // 离开作用域 vector 就销毁了
