pub mod protocol;

pub fn next_multiple(n: u32, mult: u32) -> u32 {
    let rem = n % mult;
    if rem == 0 {
        return n;
    } else {
        return n + mult - rem;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_multiple_10_5_10() {
        assert_eq!(next_multiple(10, 5), 10)
    }
    #[test]
    fn next_multiple_11_5_15() {
        assert_eq!(next_multiple(11, 5), 15)
    }
}
