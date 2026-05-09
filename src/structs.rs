mod structs {
    fn run() {}
    use core::fmt;

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn new(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    pub struct User {
        pub name: String,
        pub age: u32,
    }

    impl User {
        pub fn new(name: String, age: u32) -> Self {
            User { name, age }
        }
    }

    impl std::fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}: {})", self.name, self.age)
        }
    }
}
