use crate::grammar::*;
impl CN {
    pub fn classifier(word: String) -> &'static str {
        return "个"; //ge4
    }
    pub fn pronoun(
        person: &Person,
        number: &Number,
        gender: &Gender,
        referent: &Referent,
    ) -> &'static str {
        match (person, number, gender, referent) {
            // First-person pronouns
            (Person::First, Number::Singular, _, _) => "我", // I, me
            (Person::First, Number::Plural, _, _) => "我们", // We, us

            // Second-person pronouns
            (Person::Second, Number::Singular, _, _) => "你", // You (singular)
            (Person::Second, Number::Plural, _, _) => "你们", // You (plural)

            // Third-person pronouns
            (Person::Third, Number::Singular, Gender::Male, Referent::General) => "他", // He, him
            (Person::Third, Number::Singular, Gender::Female, Referent::General) => "她", // She, her
            (Person::Third, Number::Singular, _, Referent::Object) => "它", // It (object)
            (Person::Third, Number::Singular, _, Referent::Animal) => "牠", // Animal-specific "it"

            (Person::Third, Number::Plural, Gender::Male, Referent::General) => "他们", // They (male/mixed group)
            (Person::Third, Number::Plural, Gender::Female, Referent::General) => "她们", // They (female group)
            (Person::Third, Number::Plural, _, Referent::Object) => "它们", // They (things, objects)
            (Person::Third, Number::Plural, _, Referent::Animal) => "牠们", // They (animals)
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
