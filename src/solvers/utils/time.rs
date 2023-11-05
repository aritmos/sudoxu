/// Prints how long the inner block took to execute.
#[macro_export]
macro_rules! time {
    ($f: expr) => {
        let t = std::time::Instant::now();
        $f;
        let dt = t.elapsed();
        println!("\nFinished in {dt:?}");
    };
}

pub use time;
