use super::Person;
use super::Sex;

fn use_super() {
    let _p = Person {
        name: "".to_string(),
        age: 0,
        sex: Sex::MALE,
        email: "".to_string(),
    };
}