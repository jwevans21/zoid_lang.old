#[macro_export]
macro_rules! init {
    ($fname:literal) => {
        const FNAME: &'static str = $fname;
        const INPUT: &'static str = include_str!($fname);
    };
}