trait Problem {
    type Domain;
    type CoDomain: PartialOrd;

    fn value(argument: Self::Domain) -> Self::CoDomain;
}

mod happy_cat;
mod griewank;
