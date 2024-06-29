use std::time::{Duration, Instant};

/// test the time of function
/// # Examples
/// ```
/// use algori::test_time;
/// use algori::sorting::insertion_sort;
/// let mut a = [1,4,6,7,2,2,1,4,65,6];
/// test_time!(insertion_sort(&mut a));
/// ```
#[macro_export]
macro_rules! test_time {
    ($func:expr) => {
        let now = Instant::now();
        $func;
        println!(
            "Using\t{}\tseconds\n\t{}\tnanos",
            now.elapsed().as_secs(),
            now.elapsed().as_nanos()
        );
    };
}
