use std::collections::HashMap;

trait Stringer {
    fn to_string(&self) -> String;
}

struct Person {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}

impl Stringer for Person {
    fn to_string(&self) -> String {
        "".to_string()
    }
}

fn use_map() {
    let mut mp: HashMap<String, &str> = HashMap::new();
    mp.insert(String::from("a"), "a");

    println!("{}", mp.get("a").unwrap());
}

fn use_vec() {
    let frs = vec!["banana", "apple", "orange", "kiwi"];

    let first = frs.get(0);
    println!("{:?}", first);

    // pick the 99th item, which is non-existent:
    let non_existent = frs.get(99);
    println!("{:?}", non_existent);

    for &index in [0, 22, 99].iter() {
        match frs.get(index) {
            None => { println!("no fruit") }
            // add exception to Global Some
            Some(&"kiwi") => { println!("no one love kiwi") }
            Some(name) => { println!("fruit: {}", name) }
        }
    }
    let a_number: Option<u8> = Some(8);
    if let Some(7) = a_number {
        println!("ok")
    }

    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy");

    let b = Some("val8e");
    // panic with custom msg
    assert_eq!(b.expect("Good"), "value")
}

#[derive(Debug)]
struct DivisionZeroError;

fn safe_division(a: f64, b: f64) -> Result<f64, DivisionZeroError> {
    if b == 0.0 {
        Err(DivisionZeroError)
    } else {
        Ok(a / b)
    }
}

fn use_vec_() {
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("third element is {}", third);

    match v.get(2) {
        None => {
            eprintln!("nope")
        }
        Some(t) => {
            println!("Third element is {}", t)
        }
    }

    // mutable after immutable usage (line 73)
    v.push(6);
}

fn use_string() {
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
}

fn use_map_() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut _scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}

mod poly {
    use std::any::Any;
    use std::f64::consts::PI;
    use std::vec::Vec;

    struct Rectangle {
        width: u32,
        height: u32,
    }

    struct Circle {
        x: u32,
        y: u32,
        radius: u32,
    }

    trait IShape: Any + 'static {
        fn area(&self) -> f32;
        fn to_string(&self) -> String;
        fn as_any(&self) -> &dyn Any;
    }

    impl IShape for Rectangle {
        fn area(&self) -> f32 { (self.height * self.width) as f32 }
        fn to_string(&self) -> String {
            format!("Rectangle -> width={} height={} area={}",
                    self.width, self.height, self.area())
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl IShape for Circle {
        fn area(&self) -> f32 { (self.radius * self.radius) as f32 * PI as f32 }
        fn to_string(&self) -> String {
            format!("Circle -> x={}, y={}, area={}",
                    self.x, self.y, self.area())
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    fn a() {
        let rect = Box::new(Rectangle { width: 4, height: 6 });
        let circle = Box::new(Circle { x: 0, y: 0, radius: 5 });
        let mut v: Vec<Box<dyn IShape>> = Vec::new();
        v.push(rect);
        v.push(circle);
        for i in v.iter() {
            if let Some(s) = i.as_any().downcast_ref::<Rectangle>() {
                println!("downcast - Rectangle w={}, h={}", s.width, s.height);
            } else if let Some(s) = i.as_any().downcast_ref::<Circle>() {
                println!("downcast - Circle x={}, y={}, r={}", s.x, s.y, s.radius);
            } else {
                println!("invaild type");
            }
        }
    }
}

mod or {
    use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

    #[derive(Debug)]
    struct Employee {
        name: String,
        salary: i32,
    }

    impl Ord for Employee {
        fn cmp(&self, rhs: &Self) -> Ordering {
            self.salary.cmp(&rhs.salary)
        }
    }

    impl PartialOrd for Employee {
        fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
            Some(self.cmp(rhs))
        }
    }

    impl Eq for Employee {}

    impl PartialEq for Employee {
        fn eq(&self, rhs: &Self) -> bool {
            self.salary == rhs.salary
        }
    }

    fn m() {
        let mut v = vec![
            Employee { name: String::from("Bob"), salary: 2048 },
            Employee { name: String::from("Alice"), salary: 3208 },
            Employee { name: String::from("Tom"), salary: 2359 },
            Employee { name: String::from("Jack"), salary: 4865 },
            Employee { name: String::from("Mary"), salary: 3743 },
            Employee { name: String::from("Hao"), salary: 2964 },
            Employee { name: String::from("Chen"), salary: 4197 },
        ];

        // 用for-loop找出薪水最多的人
        let mut e = &v[0];
        for i in 0..v.len() {
            if *e < v[i] {
                e = &v[i];
            }
        }
        println!("max = {:?}", e);
        // 使用标准的方法
        println!("min = {:?}", v.iter().min().unwrap());
        println!("max = {:?}", v.iter().max().unwrap());
        // 使用标准的排序方法
        v.sort();
        println!("{:?}", v);
    }
}
