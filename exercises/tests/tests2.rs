// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

fn funciton_return_one() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use crate::funciton_return_one;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(funciton_return_one(), 1);
    }
}
