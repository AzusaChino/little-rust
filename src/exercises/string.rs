#[cfg(test)]
mod tests {
    use std::{fmt::Display, usize};

    #[test]
    pub fn concat_string() {
        by_moving();
        by_cloning();
        by_mutating();
    }

    fn by_moving() {
        let hello = "hello, ".to_owned();
        let world = "world!";
        // move hello, world to new variable handle
        let hello_world = hello + world;
        println!("{}", hello_world);
    }

    fn by_cloning() {
        let hello = "hello, ".to_owned();
        let world = "world!";

        // clone a new handle
        let hello_world = hello.clone() + world;
        println!("{}", hello);
        println!("{}", hello_world);
    }

    fn by_mutating() {
        // mutate the origin data
        let mut hello = "hello, ".to_owned();
        let world = "world!";

        hello.push_str(world);
        println!("{}", hello);
    }

    struct Demo;

    impl Display for Demo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "demo")
        }
    }

    #[test]
    fn format() {
        let color = "red";
        let favor = format!("{} is my favorite color", color);
        let _favor = format!("{c} is my favorite color", c = color);
        println!("{}", favor);

        let d = Demo {};
        println!("{}", format!("{} is demo?", d));

        let ddg = format!("{0}, {0}, {1}", "duck", "goose");
        let intro = format!(
            "my name is {sur}, {fore}, {sur}",
            sur = "bond",
            fore = "james"
        );

        println!("{} {}", ddg, intro);

        println!("{}", format!("{1} {} {0} {}", "a", "b"));
        println!("{}", format!("{:.*}", 2, 1.123435));
    }

    #[test]
    fn default() {
        // default.rs -> default_impl! { i32, 0, "Returns the default value of `0`" }
        let foo: i32 = Default::default();
        println!("{}", foo);
    }

    struct NameLen<'a> {
        name: std::borrow::Cow<'a, str>,
        length: usize,
    }

    impl<'a> NameLen<'a> {
        fn new<S>(name: S) -> Self
        where
            S: Into<std::borrow::Cow<'a, str>>,
        {
            let name: std::borrow::Cow<'a, str> = name.into();
            Self {
                length: name.len(),
                name,
            }
        }
    }

    #[test]
    fn structs() {}
}
