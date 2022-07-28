#[cfg(test)]
mod test {
    // From和Into是互逆的转换，如果T实现了From，标准库会自动实现Into
    // impl<T, U> Into<U> for T
    // where
    //     U: From<T>,
    // {
    //     fn into(self) -> U {
    //         U::from(self)
    //     }
    // }

    fn iter_bytes<T: AsRef<[u8]>>(arg: T) {
        for i in arg.as_ref() {
            println!("{}", i);
        }
    }
    #[test]
    fn main() {
        let s: String = String::from("This is a string");
        let v: Vec<u8> = vec![1, 2, 3];
        let c: &str = "hello";

        iter_bytes(s);
        iter_bytes(v);
        iter_bytes(c);
    }

    #[test]
    fn f() {
        let s: &'static str = "Hello";
        let _str1: String = s.into();
        let _str2: String = String::from(s);
    }

    use std::ops::Add;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Complex {
        real: i32,
        imaginary: i32,
    }

    // 重载算术符号
    impl Add for Complex {
        type Output = Complex;
        fn add(self, other: Complex) -> Complex {
            Complex {
                real: self.real + other.real,
                imaginary: self.imaginary + other.imaginary,
            }
        }
    }

    #[test]
    fn c() {
        let c1 = Complex {
            real: 1,
            imaginary: 2,
        };
        let c2 = Complex {
            real: 2,
            imaginary: 4,
        };
        println!("{:?}", c1 + c2);
    }
}
