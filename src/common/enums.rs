#![allow(unused)]

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
}
