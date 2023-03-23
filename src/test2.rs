use std::error::Error;
use std::fmt::Display;
use std::io;

const TAG: &str = "test2 ";
#[macro_export]
macro_rules! vec_xiaoyu {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            println!("vec_xiaoyu Vec::new()");
            $(
                temp_vec.push($x);
                println!("vec_xiaoyu {}",$x);
            )*
            println!("vec_xiaoyu return Vec");
            temp_vec
        }
    };
}



pub fn test(){
    // ownership
    test1();
    test3();

    testSlice();

    testStruct();

    testEnum();

    testOption();

    testVector();

    testCrateMod();

    testStringPush();
    
    testHashMap();
    
    testPanic();
    
    testResult();
    
    testGenerics();
    
    testTrait();

    testTraitBound();

    testLifeCycle();

    testMacroRules();

}

fn testMacroRules() {
    let v: Vec<u32> = super :: vec_xiaoyu![1, 2, 3, 10];
    dbg!(v);


// 测试macro derive
    Pancakes::new().testMethod();
    Pancakes::hello_macro();
    Pancakes::hello_macro_11();

    // Pancakes{}.hello_macro_11();

    pub trait HelloMacro {
        fn hello_macro();
    }
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes {
    }

    impl Pancakes{
        pub fn new() -> Self {
            Pancakes{}
        }

        pub fn testMethod(&self){
            dbg!(" testMethod  for  Pancakes");
        }
    }

    pub trait HelloMacro_11 {
        fn hello_macro_11();
    }

    impl HelloMacro_11 for  Pancakes{
        fn hello_macro_11() {
            dbg!(" impl HelloMacro_11 for  Pancakes");
        }
    }

}

//避免悬垂引用（dangling references）
fn testLifeCycle() {
    //'static，其生命周期能够存活于整个程序期间。
    let s: &'static str = "I have a static lifetime.";



    dbg!(longest_with_an_announcement("sfjfj","fjdjfjf", 100.0));

    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
        where
            T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }


    let r;
    {
        let x = 5;
        r = &x;
    }
    //  下面这句会报错   ^^ borrowed value does not live long enough
    // println!("r: {}", r);


    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

// help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
   // 提示文本揭示了返回值需要一个泛型生命周期参数，
    // 因为 Rust 并不知道将要返回的引用是指向 x 或 y。
// 事实上我们也不知道，因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！
    //     fn longest(x: &str, y: &str) -> &str {
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
            x
        } else {
            y
        }
    }



    let string1 = String::from("long string is long");
    let result1;
    {
        let string2 = String::from("xyz");
        result1 = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result1);
    }
    // /**
    // longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致。因此，
    // 借用检查器不允许下面的代码，因为它可能会存在无效的引用。
    // result1的生命周期和string2 一致
    //  **/
    // println!("The longest string is {}", result1); // ^^ borrowed value does not live long enough


    //我们为参数 x 和返回值指定了生命周期参数 'a，
    // 不过没有为参数 y 指定，因为 y 的生命周期与参数 x 和返回值的生命周期没有任何关系。
    fn longest_1<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
    let string1 = String::from("long string is long");
    let result2;
    {
        let string2 = String::from("xyz");
        result2 = longest_1(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result2);
    }
    // 这个可以跑，是因为 longest_1返回值的生命周期等于 string1的生命周期
    println!("The longest string is {}", result2); // ^^ borrowed value does not live long enough


    struct ImportantExcerpt<'a> {
        part: &'a str, // 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };



   println!("{}", first_word("heooo fddnsfbgdn"));
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

}

fn testTraitBound() {
    use std::fmt::Display;

    let pair = Pair{
        x : "dsnvfngag", y: "fjdjshgh"
    };
    dbg!(pair.cmp_display());

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    //只有那些为 T 类型实现了 PartialOrd trait （来允许比较） 和 Display trait （来启用打印）
    // 的 Pair<T> 才会实现 cmp_display 方法：
    impl<T: Display + PartialOrd> Pair<T> {
        // 有条件的增加方法
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

fn testTrait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    notify(&article);




    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    pub trait Summary {
        fn summarize_author(&self) -> String;

        // fn summarize(&self) -> String {
        //     String::from("(Read more...)")// 这里是默认实现
        // }

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
        // fn summarize(&self) -> String {
        //     format!("{}, by {} ({})", self.headline, self.author, self.location)
        // }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        // fn summarize(&self) -> String {
        //     format!("{}: {}", self.username, self.content)
        // }
    }
}


/**
//Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
编译器生成的单态化版本的代码看起来像这样（编译器会使用不同于如下假想的名字）：

文件名：src/main.rs

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
**/
fn testGenerics() {
    test();
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
    test_6();

    struct Point_2<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point_2<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point_2<X2, Y2>) -> Point_2<X1, Y2> {
            Point_2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn test_6() {
        let p1 = Point_2 { x: 5, y: 10.4 };
        let p2 = Point_2 { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    fn test_5() {
        let p = Point { x: 7.0, y: 19.0 };
        println!("p.distance_from_origin = {}", p.distance_from_origin());
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    impl<T> Point<T> {
        fn getX(&self) -> &T {
            &self.x
        }
    }

    fn test_4() {
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.getX());
    }

    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    struct Point_1<T, U> {
        x: T,
        y: U,
    }
    fn test_3() {
        let both_integer = Point_1 { x: 5, y: 10 };
        let both_float = Point_1 { x: 1.0, y: 4.0 };
        let integer_and_float = Point_1 { x: 5, y: 4.0 };
    }

    //字段 x 和 y 的类型必须相同，因为他们都有相同的泛型类型 T
    struct Point<T> {
        x: T,
        y: T,
    }

    fn test_2() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }

    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn test() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }

    fn largest_1<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn test_1() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_1(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_1(&char_list);
        println!("The largest char is {}", result);
    }
}


fn testResult() {
    use std::fs::File;
    use std::io::ErrorKind;
    use std::io::Read;


    test_get_result();

    fn test_get_result() -> Result<String,  Box<dyn Error>>{
        dbg!(getResult(11)?);
        dbg!(getResult(2)?);
        getResult(2)
    }

    // pub type MyResult<T, E = Error> = std::result::Result<T, E>;
    fn getResult(input : usize) -> Result<String,  Box<dyn Error>>{
        if input > 10 {
            Ok(String::from("getResult success"))
        }
        else {
            Err(format!("{}", "getResult Failed"))?
        }
    }


    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // let greeting_file = File::open("hello1.txt").unwrap();
    // dbg!(greeting_file);

    // let greeting_file = File::open("hello1.txt")
    //     .expect("hello1.txt should be included in this project");

    read_username_from_file();
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello1.txt")?;
        let mut username = String::new();
        //Result 值之后的 ? 被定义为与示例 9-6 中定义的处理 Result 值的 match 表达式有着完全相同的工作方式。
        // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
        // 如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }


    read_username_from_file_1();
    fn read_username_from_file_1() -> Result<String, io::Error> {
        let mut username = String::new();
        //File::open 调用结尾的 ? 会将 Ok 中的值返回给变量 username_file。如果发生了错误，? 运算符会使整个函数提前返回并将任何 Err 值返回给调用代码。
        // 同理也适用于 read_to_string 调用结尾的 ?。
        // ? 运算符消除了大量样板代码并使得函数的实现更简单。我们甚至可以在 ? 之后直接使用链式方法调用来进一步缩短代码
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }


    dbg!(read_username_from_file_2().unwrap());
    use std::fs;
    use std::io;
    fn read_username_from_file_2() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    //这个不行，因为？ 只有在返回类型是 Result时才可以用
    // let greeting_file = File::open("hello.txt")?;
    fn testReturnResult() -> Result<(File), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;
        Ok((greeting_file))
    }
    dbg!(testReturnResult().unwrap());
}

fn testPanic() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];
}

fn testHashMap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    //score 是与蓝队分数相关的值，应为 10。get 方法返回 Option<&V>，如果某个键在哈希 map 中没有对应的值，get 会返回 None。
    // 程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，
    // 接着调用 unwrap_or 在 score 中没有该键所对应的项时将其设置为零。
    let score = scores.get(&team_name).copied().unwrap_or(0);
    dbg!(score);



    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    // dbg!(field_value);   当 insert 调用将 field_name 和 field_value 移动到哈希 map 中后，将不能使用这两个绑定。
}

fn testStringPush() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // 不会获取s2的所有权
    println!("s2 is {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // 不会获取任何参数的所有权
    println!("s is {s}");
    println!("s1 is {s1}");


    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {s}");
    // let s = &hello[0..1]; 这个会panic ，原因是rust是UTF-8编码，一个字符可能需要两位/多位来表示； Unicode 标量值可能会由不止一个字节组成。

    for b in "Зд".bytes() {
        println!("{b}");
    }

}

fn testCrateMod() {
    use mylib :: xiaoyu;
    xiaoyu :: add(5, 6);
}

fn testVector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}


fn testOption() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
    dbg!(sum);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(six);
    dbg!(none);
}

fn testEnum() {
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    //
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    //
    // let loopback = IpAddr::V6(String::from("::1"));

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));


    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }




}

fn testStruct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(16, 0, 0);
    let origin = Point(0, 0, 0);

    println!( "{} black {}", TAG,  black.0);


    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{} rect1 is {:?}", TAG, rect1);
    dbg!(&rect1);


    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    impl Rectangle{
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle :: square(50);
    dbg!(&rect4);


}

fn testSlice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];


    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    println!( "{} word {}", TAG,  word);


    let word = first_word(&my_string_literal[..]);
    println!( "{} word {}", TAG,  word);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);

    println!( "{} word  {}", TAG,  word);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test1() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效


    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    // 所以在后面可继续使用 x

    let y = x;

} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{} {}",TAG, some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!( "{} {}", TAG,  some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处



fn test3() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len); // 这里因为前面 calculate_length 方法转入的是 & 引用，因此剋哟发生所有权转移，这里还可以用
}

fn calculate_length(s: &String) -> usize {
    s.len()
}