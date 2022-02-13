/// A function to calculate nth fibonacci number
/// ```
/// # use rust_coverage::fibonacci;
/// assert_eq!(fibonacci(0), Some(0));
/// assert_eq!(fibonacci(1), Some(1));
/// assert_eq!(fibonacci(2), Some(1));
/// assert_eq!(fibonacci(3), Some(2));
/// ```
pub fn fibonacci(n: i32) -> Option<i32> {
    match n {
        0 => Some(0),
        1 => Some(1),
        n if n >= 2 => Some(fibonacci(n - 1).unwrap() + fibonacci(n - 2).unwrap()),
        _ => None,
    }
}

#[test]
pub fn fibonacci_work() {
    assert_eq!(fibonacci(10), Some(55))
}
