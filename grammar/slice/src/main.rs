fn main() {
    let x = "hello world";
    let x_index = first_word(x);

    let y = String::from("hello world");
    let y_index = first_word(&y[..]);

    // s.clear(); // error
    println!("{}, {}", x_index, y_index);

    // 切片 指向字符串一部分内容
    // [开始索引..结束索引]
    let s = String::from("hello world");
    let s1 = &s[0..5]; // 0-4,包含0，不包含5
    let s2 = &s[6..11];
    // 语法糖
    let s3 = &s[..5];
    let s4 = &s[6..];
    let s5 = &s[..];
    println!("{}, {}, {}, {}, {}", s1, s2, s3, s4, s5);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// 返回切片
// fn first_word(s: &str) -> &str {
// 采用 &str 作为参数，即拥有了 String 又拥有了 &str 类型的参数
// 使用字符串切片直接调用函数
// 使用 String 创建 String 切片
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
