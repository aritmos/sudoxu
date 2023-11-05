#![allow(unused)]
#![allow(clippy::unusual_byte_groupings)]

use sudoxu::prelude::*;

fn main() {
    time!({
        solve_board::<btr::Backtracker>(B1_U_STR).unwrap().print();
    });
}
