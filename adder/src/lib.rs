//! The `adder` crate provides funtions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(adder::add(1, adder::add(10, 100)), 111);
//! ```

/// This function adds two numbers.
///
/// # Examples
///
/// ```
/// use adder::add;
///
/// assert_eq!(add(3, 4), 7);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// This function adds 3 numbers.
///
/// # Examples
///
/// ```
/// use adder::add3;
///
/// assert_eq!(add3(1, 10, 100), 111);
/// ```
pub fn add3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "no-panic")]
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn it_does_not_work() {
        // assert!(false)
    }

    #[test]
    fn it_works() {
        assert_eq!(add(2, 5), 7)
    }
}
