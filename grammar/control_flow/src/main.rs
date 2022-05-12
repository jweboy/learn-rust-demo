fn main() {
    let num = 3;

    // if 表达式
    if num < 2 {
        println!("condition is false");
    } else {
        println!("condition is true");
    }

    if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of the number is: {}", number);

    // loop
    // loop {
    //     println!("again!");
    // }

    let mut counter = 2;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break停止循环
        }
    };

    println!("The result is {}", result);

    // while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", a[index]);

        index += 1;
    }

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is {}", element);
    }

    // range
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LISTOFF!");
}
