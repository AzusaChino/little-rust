#![allow(unused)]

use std::collections::HashMap;

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4}    {}    {}",
            "Type", "T", "Option<T>", "Result<T, std::io::Error>"
        );
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4} {:8} {:12}",
            stringify!($t),
            std::mem::size_of::<$t>(),
            std::mem::size_of::<Option<$t>>(),
            std::mem::size_of::<Result<$t, std::io::Error>>()
        );
    };
}

#[derive(Debug)]
enum SimpleMessage {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

#[derive(Debug)]
enum Message {
    Move { x: u8, y: u8 },
    Echo(String),
    ChangeColor((u8, u8, u8)),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::Quit => {
                self.quit();
            }
            Message::Echo(s) => {
                self.echo(s);
            }
            Message::Move { x, y } => {
                self.move_position(Point { x, y });
            }
            Message::ChangeColor((x, y, z)) => {
                self.change_color((x, y, z));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::borrow::Cow;

    #[test]
    fn print_messages() {
        let messages = [
            Message::Move { x: 10, y: 30 },
            Message::Echo(String::from("hello world")),
            Message::ChangeColor((200, 255, 255)),
            Message::Quit,
        ];

        for message in &messages {
            message.call();
        }
    }

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move { x: 10, y: 15 });
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        // assert_eq!(state.quit, true);
    }

    #[test]
    fn test_size() {
        show_size!(header);
        show_size!(u8);
        show_size!(u16);
        show_size!(u32);
        show_size!(f64);
        show_size!(&u8);
        show_size!(Box<u8>);
        show_size!(&[u8]);

        show_size!(String);
        show_size!(Vec<u8>);
        show_size!(HashMap<String, String>);
        show_size!(E);
    }

    #[test]
    fn test_cow() {
        let mut cow_list = vec![
            (Cow::Borrowed("hello"), Cow::Borrowed("world")),
            (Cow::Owned(String::from("hello")), Cow::Borrowed("world")),
            (Cow::Borrowed("hello"), Cow::Owned(String::from("world"))),
            (
                Cow::Owned(String::from("hello")),
                Cow::Owned(String::from("world")),
            ),
        ];

        let (mut k, v) = cow_list.pop().unwrap();
        k.to_mut().push_str(" world");
        print_pairs((k, v));
    }

    fn print_pairs(pair: (Cow<str>, Cow<str>)) {
        println!("{}: {}", print_cow(pair.0), print_cow(pair.1));
    }

    fn print_cow(cow: Cow<str>) -> String {
        match cow {
            Cow::Borrowed(s) => format!("Borrowed: {}", s),
            Cow::Owned(s) => format!("Owned: {}", s),
        }
    }
}
