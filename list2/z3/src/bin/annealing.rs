use ::z3::annealing;
use ::z3::bin_utils::{self, Result};

fn main() -> Result {
    bin_utils::main(annealing::search)
}
