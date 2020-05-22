use ::z3::bin_utils::{self, Result};
use ::z3::genetic;

fn main() -> Result {
    bin_utils::main(genetic::search)
}
