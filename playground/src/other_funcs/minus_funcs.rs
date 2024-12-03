pub fn subtract_10(num: u32) -> u32 {
    num - 10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_five_test() {
        let x = 100;
        let y = subtract_10(x);
        println!("x an y from test are: {} {}", x, y);
        assert_eq!(y, 90);
    }
}
