mod board;
use board::Board;

mod solvers;
use solvers::utils::boards::SUDOKU_1_UNSOLVED_STR;

fn main() {
    use std::ptr;

    let mut array = [0, 1, 2, 3];

    let (x, y) = array.split_at_mut(2);
    let x = x.as_mut_ptr().cast::<[u32; 2]>(); // this is `array[0..2]`
    let y = y.as_mut_ptr().cast::<[u32; 2]>(); // this is `array[2..4]`

    unsafe {
        ptr::swap(x, y);
        assert_eq!([2, 3, 0, 1], array);
    }
    println!("{b}");
}
