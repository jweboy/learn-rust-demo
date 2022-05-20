// 标准库中的定义
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let a = Some(4);
    let b = Some("Some other");
    let c: Option<i32> = None;

    // Option<T> 和 T 是两个不同类型
    let d: i8 = 4;
    let e: i8 = 6;
    // let e: Option<i8> = Some(5);
    let sum = d + e;
}
