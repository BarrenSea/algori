/// test the time of function
/// # Examples
/// ```
/// use algori::test_time;
/// use algori::sorting::insertion_sort;
/// let mut a = [1,4,6,7,2,2,1,4,65,6];
/// test_time!("Insertion Sort",insertion_sort(&mut a,|a,b|a<=b));
/// ```
#[macro_export]
macro_rules! test_time {
    ($title:literal,$func:expr) => {
        let now = std::time::Instant::now();
        $func;
        println!(
            "Job:\t{}\nUsing\t{}\tseconds\n\t{}\tnanos",
            $title,
            now.elapsed().as_secs(),
            now.elapsed().as_nanos()
        );
    };
}
