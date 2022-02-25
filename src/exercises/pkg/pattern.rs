mod matching {
    struct Point {
        x: i32,
        y: i32,
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    fn a() {
        let x = 'c';
        match x {
            'a'..='j' => println!("early"),
            'k'..='z' => println!("early"),
            _ => println!("else")
        }
    }

    fn b() {
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }

    fn c() {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x, y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            ),
        }
    }

    fn d() {
        fn main() {
            let numbers = (2, 4, 8, 16, 32);

            match numbers {
                (first, _, third, _, fifth) => {
                    println!("Some numbers: {}, {}, {}", first, third, fifth)
                }
            }
        }
    }
}

mod other {
    enum Message {
        Hello { id: i32 },
    }

    fn a() {
        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}