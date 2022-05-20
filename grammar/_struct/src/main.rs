// 结构体 struct
// 该 struct 拥有其所有的数据
// 只要 struct 有效，里面的字段数据都有效
struct User {
    username: String,
    // othername: &str, // struct 可以存放引用，只要struct 有效，里面的引用都有效，存在引用不使用生命周期会报错，如这行
    email: String,
    sign_in_url: String,
    active: bool,
}

// tuple struct
struct Color(i32, i32, i32);

// unit like struct 没有任何字段的结构体

fn main() {
    // 一旦 struct 可变，那么他的所有字段都是可变的
    let mut user1 = User {
        email: String::from("123456789@qq.com"),
        username: String::from("John Doe"),
        active: false,
        sign_in_url: String::from("http://example.com"),
    };
    user1.email = String::from("12345xx9@qq.com");
    user1.active = true;

    // 更新语法
    let user2 = User {
        email: String::from("1x6789@qq.com"),
        username: String::from("Sally"),
        ..user1
    };

    let black = Color(0, 0, 0);

    let w = 500;
    let h = 100;
    println!("{}", area(w, h));

    let rect = (100, 30);
    println!("{}", area_tuple(rect));

    let rect = Rect {
        width: 200,
        height: 300,
    };
    println!("{}", area_struct(&rect));
    // 调用方法时，rust 会自动添加 & &mut *，以便对象可以匹配方法的签名
    println!("{}", rect.area());
    // 输出格式化
    println!("{:#?}", rect);
    println!("{:?}", rect);

    // 关联函数调用，类似JS的静态函数
    let rect1 = Rect::square(66);
    println!("{:?}", rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 元组形式
fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

// 结构体
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// 结构体方法
impl Rect {
    // 带有 self 的就是方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 关联函数
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

// 结构体形式
fn area_struct(rect: &Rect) -> u32 {
    rect.width * rect.height
}

// 作为函数返回值
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_url: String::from("http://example.com"),
    }
}
