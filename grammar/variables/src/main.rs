// const MAX_POINTS: u32 = 200_000;

fn main() {
    println!("Hello, world!");

    // let声明变量
    // 变量不可变，可通过 mut 成为可变
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    // shadowing 隐藏
    // 相同名字声明新的变量，新的变量就会覆盖之前声明的同名变量
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    // const声明常量
    // 常量永远不可变，不能用mut
    // 任何作用域有效
    // 只能绑定到常量表达式，不可绑定到函数调用结果
    const MAX_POINTS: u32 = 100;
    println!("The max points is {}", MAX_POINTS);
}
