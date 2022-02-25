#![allow(dead_code)]
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;

fn main() {
    //  --> src\bin\panic.rs:2:5
    //   |
    // 2 |     None.unwrap();
    //   |     ^^^^---------
    //   |     |
    //   |     this method call resolves to `T`
    //   |     cannot infer type for type parameter `T` declared on the enum `Option`
    // None.unwrap();
    //
    assert_eq!(None.unwrap_or("cat"), "cat");
}

// fn test_diff_lifetime() {
//     let m = String::from("abracadabra");
//     let result;
//     {
//         let s = String::from("shazam");
//         result = longest_word(&m, &s);
//     }
//     println!("magic: {}", result)
// }

// declare generic lifetime inside angle brackets
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Cannot compile, must add lifecycle
fn longest_word_fail_compile<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn read_file(path: PathBuf) {
    let f = File::open(path.clone());

    let _f = match f {
        Ok(ff) => ff,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let __f = File::open(path.clone()).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path.clone())
                .unwrap_or_else(|error| panic!("create file error {:?}", error))
        } else {
            panic!("error open file: {:?}", error)
        }
    });
}
