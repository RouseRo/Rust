/// Function: add_five
///
/// # Arguments (num: u32)
///
/// # Returns u32
///
/// # Example
/// ```
/// let x = 5;
/// let y = add_five(x);
/// ```
///
/**
*
* This is a multiline block comment
*
*/
pub fn add_five(num: u32) -> u32 {
    num + 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_five_test() {
        let x = 100;
        let y = add_five(x);
        println!("x an y from test are: {} {}", x, y);
        assert_eq!(y, 104);
    }
}
