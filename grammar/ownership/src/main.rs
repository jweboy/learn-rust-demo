fn main() {
    // 变量作用域 scope
    {
        // s 不可用
        let s = "hello"; // s 可用
                         // 可以对 s 进行操作
                         // s.push_str(", world!"); // 字符串字面量不可变（编译时就知道了具体内容大小，其文本内容被硬编码到执行文件里）
        println!("{}", s);
    } // s 作用域到此结束 s 不可用 （其实就是通过 drop 函数将内存交还给操作系统）

    // String 类型
    // 存在 heap （堆上）
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);

    // 移动所有权
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1);

    // clone 深拷贝 heap（堆）上的数据
    let s1 = String::from("Hello");
    let _s2 = s1.clone();
    println!("{}, {}", s1, s2);

    // stack (栈) 复制，实际是实现了 copy trait
    // 任何简单标量的组合类型都实现了 copy trait（如：整数类型、bool、char、tuple）
    // 任何需要分配内存的都不能 copy
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
}
