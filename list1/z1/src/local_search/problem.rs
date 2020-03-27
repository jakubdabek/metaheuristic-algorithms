#![allow(type_alias_bounds)]

use crate::assoc_fcs;

pub trait Domain {
    type Argument: Clone;
    type Value: PartialOrd;

    fn random(scale: f64) -> Self::Argument;
    fn random_near(point: &Self::Argument, scale: f64) -> Self::Argument;
}

pub type ProblemArgument<P: Problem> = assoc_fcs!(P: Problem->Domain->Argument);
pub type ProblemValue<P: Problem> = assoc_fcs!(P: Problem->Domain->Value);

pub trait Problem {
    type Domain: Domain;

    fn value(argument: &ProblemArgument<Self>) -> ProblemValue<Self>;
}
