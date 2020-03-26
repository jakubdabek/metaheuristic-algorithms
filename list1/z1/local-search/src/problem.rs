#[macro_export]
macro_rules! assoc_fcs {
    ($trait:ident -> $outer:ident -> $inner:ident) => {
        <<Self as $trait>::$outer as $outer>::$inner
    };
}

pub trait Domain {
    type Argument;
    type Value: PartialOrd;

    fn random(scale: f64) -> Self::Argument;
    fn random_near(point: Self::Argument, scale: f64) -> Self::Argument;
}

pub trait Problem {
    type Domain: Domain;

    fn value(argument: assoc_fcs!(Problem->Domain->Argument))
        -> assoc_fcs!(Problem->Domain->Value);
}
