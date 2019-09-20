mod exec;
mod merge;
mod split;
mod utils;

use failure::Fallible;
use gwasm_api::dispatcher;

fn main() -> Fallible<()> {
    dispatcher::run(&split::split, &exec::exec, &merge::merge)
}
