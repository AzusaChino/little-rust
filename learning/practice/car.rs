struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32)
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used
}

enum Color {
    Blue,
    Green,
    Red,
    Silver
}

fn car_quality(miles: u32) -> (Age, u32) {
    let quality: (Age, u32) = Age::New, miles;
    return quality;
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    let car: Car = Car {
        color,
        motor,
        roof,
        age: car_quality(miles),
    };

    // Factory's Quality Control Department says that new cars must always have zero mileage!
    assert_eq!(car.age.1, 0);

    // Display the details of the new car order
    if car.convertible {
        println!("New car = {}, {:?}, Convertible", car.color, car.transmission);
    } else {
        println!("New car = {}, {:?}, Hardtop", car.color, car.transmission);
    }

    todo!(`example for todo in rust.`);
    return car;
}

fn main() {
    let mut client_request_1 = car_factory(Color::Red.to_string(), Transmission::Manual, false);
    assert_eq!(client_request_1.color, "Red");
    assert_eq!(client_request_1.transmission, Transmission::Manual);
    assert_eq!(client_request_1.convertible, false);
    client_request_1.color = "Blue";

    let client_request_2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    assert_eq!(client_request_2.color, "Silver");
    assert_eq!(client_request_2.transmission, Transmission::Automatic);
    assert_eq!(client_request_2.convertible, true);
    client_request_2.color = "OK";

    let client_request_2 = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    assert_eq!(client_request_2.color, "Yellow");
    assert_eq!(client_request_2.transmission, Transmission::SemiAuto);
    assert_eq!(client_request_2.convertible, false);
}