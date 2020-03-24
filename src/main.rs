mod hello_world;
use hello_world::hello_world;

mod primitives;
use primitives::*;

mod utils;

mod borrowing;

fn main() {
    hello_world();
    lit_and_ops::run();
    tuples::run();
    arrays::run();
    borrowing::run();
}
