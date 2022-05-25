use std::collections::HashMap;

fn main() {
    // HashMap 数据存储在 heap (堆) 中，必须是同构的，K,V 必须是同类型
    let mut scores = HashMap::new();

    scores.insert(String::from("hello"), String::from("world"));

    println!("scores: {:?}", scores);

    let teams = vec![String::from("Yellow"), String::from("Blue")];
    let scores = vec![10, 30, 50];

    let scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("scoress: {:?}", scores);

    // 对于实现了 Copy trait 的类型，值会被复制到 HashMap
    // 对于拥有所有权的值，值的所有权会被移动到 HashMap

    let mut map = HashMap::new();
    let name = String::from("Sally");

    // map.insert(String::from("hello"), 10);
    // map.insert(String::from("world"), 20);
    // map.insert(name, 40);
    // 值引用插入到 HashMap ，值本身不会移动
    map.insert(&name, 40);

    println!("{}", name);
    println!("{}", &name);

    let team_name = String::from("world");
    // 获取 HashMap 中的值
    let key = map.get(&team_name);

    match key {
        Some(key) => println!("{}", key),
        None => println!("not found"),
    }

    //  遍历
    for (k, v) in scores {
        println!("{}, {}", k, v);
    }

    {
        // 更新 - 替换现有 V
        let mut scores = HashMap::new();

        scores.insert(String::from("Yellow"), 1);
        scores.insert(String::from("Yellow"), 2);

        println!("{:?}", scores);
    }
    {
        // 更新 - 只有 K 不对应任何情况才插入 V
        let mut scores = HashMap::new();

        scores.insert(String::from("Yellow"), 1);
        scores.entry(String::from("Yellow")).or_insert(40);
        scores.entry(String::from("Yellows")).or_insert(40);
        println!("{:?}", scores);
    }
}
