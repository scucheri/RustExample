pub mod xiaoyu;

fn add_100(num : usize) -> usize{
    num + 100
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = xiaoyu :: add(2, 2);
        println!("result is {}",result);
        assert_eq!(result, 4);
    }

        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }



    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle{
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }


    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }

            Guess { value }
        }
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // 这个意思是panic错误信息要包含这个字符串
    fn greater_than_100() {
        Guess::new(200);
    }


    #[test]
    fn it_works_1() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }


    fn add_two(num : usize)-> usize{
        num + 2
    }

    #[test]
    #[ignore]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }


    #[test]
    fn test_super_mod_fun() {
        assert_eq!(200, super:: add_100(100));
    }

}






