use rand;

/// calculates the sum of the two given numbers
///
/// # Example
///
/// ```
///  use loli_lib::add;
///
///  let sum = add(3, 5);
/// ```
///
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_subtract() {
        let result = subtract(4, 2);
        assert_eq!(result, 2);
    }
}
