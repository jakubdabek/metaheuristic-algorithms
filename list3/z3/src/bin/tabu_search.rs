use ::z3::bin_utils::{self, Result};
use z3::tabu_search;

fn main() -> Result {
    bin_utils::main(tabu_search::search)
}
