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