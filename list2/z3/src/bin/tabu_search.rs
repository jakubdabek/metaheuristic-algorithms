use z3::tabu_search;
use ::z3::bin_utils::{self, Result};

fn main() -> Result {
    bin_utils::main(tabu_search::search)
}
