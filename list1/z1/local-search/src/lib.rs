trait Problem {
    type Domain;
    type CoDomain: PartialOrd;

    fn value(argument: Self::Domain) -> Self::CoDomain;
}

mod griewank;
mod happy_cat;
