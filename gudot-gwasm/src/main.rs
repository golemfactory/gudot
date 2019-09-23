mod exec;
mod merge;
mod split;

type Result<T> = std::result::Result<T, String>;

use failure::Fallible;
use gwasm_api::dispatcher;

fn main() -> Fallible<()> {
    dispatcher::run(split::split, exec::exec, merge::merge)
}
