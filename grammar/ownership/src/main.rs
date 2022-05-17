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

    {
        let s1 = String::from("Hello");
        let len = calculate_length(&s1); // 引用 s1，但是并不拥有 s1
        println!("{}, {}", s1, len);

        fn calculate_length(s: &String) -> usize {
            // 这里的参数 & 就是借用（借用就是把引用当作函数参数）
            // s.push_str(", world"); // error 借用默认也是不可变的
            s.len()
        }
    }

    {
        // 可变引用
        let mut s1 = String::from("Hello");
        let len = calculate_length(&mut s1); // 引用 s1，但是并不拥有 s1
        println!("{}, {}", s1, len);

        fn calculate_length(s: &mut String) -> usize {
            // 这里的参数 & 就是借用（借用就是把引用当作函数参数）
            s.push_str(", world"); // error 借用默认也是不可变的
            s.len()
        }
    }

    // {
    //     // 可变引用(&mut)只能有一个，防止数据竞争
    //     let mut s = String::from("Hello world");
    //     let s1 = &mut s;
    //     // let s2 = &mut s;
    // }

    {
        // 创建新的作用域，允许非同时创建多个可变引用
        let mut c = String::from("Hello world");
        {
            let c1 = &mut c;
        }
        let c2 = &mut c;
    }

    // 不可同时拥有可变引用和不可变引用
    let mut d = String::from("Hello world");
    let d1 = &d;
    let d2 = &mut d; // error
    println!("{}, {}", d1, d2);

    // 保证引用永远都不是悬空引用
    // dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// 规则
// 1. 一个可变的引用 / 任意数量的不可变引用
// 2. 引用一直有效
