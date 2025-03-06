// WIP
pub struct CN {}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Singular,
    Plural,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}

pub enum Gender {
    Male,
    Female,
}
pub enum Formality {
    Informal,
    Formal,
    Literary,
    Honorific,
}

pub enum Referent {
    General,

    Animal,
    Object,
}
