pub fn s004(a: i32, b: i32, c: i32) -> i32 {
    100 * a + 10 * b + c
}

#[cfg(test)]
mod s004_tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(s004(1, 2, 8), 128)
    }
}