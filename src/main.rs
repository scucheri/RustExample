mod test1;
mod test2;
use std::io;


fn main() {
    test2 :: test();

    println!("Hello, world!");


    let mut str1_0 = "julia book".to_string();
    let mut str1_1 = "124345";
    let mut f4 =   |x: &str, y: &str| {
        println!("x:{:?} y:{:?}  str1_0: {:?}", x, y, str1_0);
    };

    let mut f5 =  move |x: &str, y: &str| {
        println!("x:{:?} y:{:?}  str1_0: {:?}", x, y, str1_0);
    };
    let _str2 = f5(" 2013", "1235509060");
    // let _str2 = f4(" 2013", "1235509060");

    // println!("str2:{:?}", str1_0);  //这里会报错，borrow of moved value: `str1_0`


    testFun();


    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);


    testLifecycleNote();

    fn main() {
        let x = 1;
        // 闭包函数
        let sum = |y| x + y;
        //如果不想等则抛出异常
        let sumval = sum(2);
        println!("sumval:{:?} ", sumval);
        assert_eq!(3, sum(2));
    }
    main();

  

    // test1 :: main();

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x : u32|{ x + 1 };
    let add_one_v4 = |x : u32|x + 1;


    struct Cacher<T>
        where
            T: Fn(u32) -> u32,
    {
        query: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
    {
        fn new(query: T) -> Cacher<T> {
            Cacher {
                query,
                value: None,
            }
        }

        // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut c = Cacher::new(|x| x * 10);
    println!("{}", c.value(32));
    println!("{}", c.value(33));


    //equal_to_x 闭包的好处是可以直接获取到x
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));


    // let x = 4;
    // fn equal_to_x(z: i32) -> bool {
    //     z == x  // 函数中不能获取到x；
    // }
    // let y = 4;
    // assert!(equal_to_x(y));

  testCopy();


  testOwnerShipMove();

   testClosure();

    // testArray();


   testFunReturn();
    
    testIf();

    testLoop();

    testWhile();
    
    testFor();
}

fn testFor() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }


    //.rev 用来反转顺序
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn testWhile() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// break continue默认是退出最里层的循环   'counting_up: loop { 这样可以给循环做标记
fn testLoop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn testIf() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn testFunReturn() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

}

fn testFun() {
    let mut num = 5;
    let mut f1 = move |x: i32| num = x + num;
    let data1 = f1(2_i32);
    println!("num:{:?} ", num);



    let mut num = 5;
    let mut f2 = |x: i32| num = x + num;
    let data2 = f2(2_i32);
    println!("num:{:?} ", num);


    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    println!("num:{:?} ", num);


    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    println!("num:{:?} ", num);
}

fn testLifecycleNote() {
    //这个也是编译不过的，因为在编译期间是无法决定返回的是x还是y，所以返回值的生命周期是无法决定的，需要用生命周期注解来解决这个问题
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    //其他任何编译器无法决定生命周期的情况都需要加注解
    fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn testOwnerShipMove() {
    //如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 关键字，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程。
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

}

fn testCopy() {
    //仅实现 FnOnce 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用：
    fn fn_once<F>(func: F)
        where
            F: FnOnce(usize) -> bool,
    {
        println!("{}", func(3));
        // println!("{}", func(4));
    }

    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()});



    //下面代码中，func 的类型 F 实现了 Copy 特征，调用时使用的将是它的拷贝，所以并没有发生所有权的转移。
    fn fn_once_1<F>(func: F)
        where
            F: FnOnce(usize) -> bool + Copy,// 改动在这里
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }
    let x = vec![1, 2, 3];
    fn_once_1(|z|{z == x.len()});
}

fn testClosure() {
    let mut s = String::new();
    //虽然报错了，但是编译器给出了非常清晰的提示，想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型，也就是 update_string 要修改为 mut update_string：
    let mut update_string =  |str| s.push_str(str);
    update_string("hello");
    println!("{:?}",s);


    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    exec(update_string);
    println!("{:?}", s);

    fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
        f("hello")
    }
}

fn testArray() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
