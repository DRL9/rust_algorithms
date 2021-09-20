//! 求两个数的最大公约数

/// 欧几里得， 辗转相除法
pub fn euclidean(m: u32, n: u32) -> u32 {
    assert!(m > 0);
    assert!(n > 0);
    if m < n {
        return euclidean(n, m);
    }
    let r = m % n;
    if r == 0 {
        return n;
    }
    euclidean(n, r)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euclidean() {
        let test_cases = [
            [13, 13, 13],
            [37, 600, 1],
            [20, 100, 20],
            [624129, 2061517, 18913],
        ];
        for case in test_cases {
            assert_eq!(euclidean(case[0], case[1]), case[2])
        }
    }
}
