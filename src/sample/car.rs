#[derive(Debug)]
pub struct Car {
    pub color: Color,
    pub motor: Transmission,
    pub roof: bool,
    pub age: (Age, u32),
}

#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
pub enum Age {
    New,
    Used,
}

#[derive(Debug)]
pub enum Color {
    Blue,
    Green,
    Red,
    Silver,
}
