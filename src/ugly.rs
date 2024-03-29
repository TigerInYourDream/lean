pub fn nth_ugly_number(n: i32) -> i32 {
    let mut dp = vec![0; n as usize];
    dp[0] = 1;
    let mut i2 = 0;
    let mut i3 = 0;
    let mut i5 = 0;

    for i in 1..n {
        dp[i as usize] = std::cmp::min(std::cmp::min(dp[i2] * 2, dp[i3] * 3), dp[i5] * 5);
        if dp[i as usize] == dp[i2] * 2 {
            i2 += 1
        }
        if dp[i as usize] == dp[i3] * 3 {
            i3 += 1
        }
        if dp[i as usize] == dp[i5] * 5 {
            i5 += 1
        }
    }

    dp[n as usize - 1]
}

#[cfg(test)]
mod test {
    #[test]
    fn test_ugly() {
        use super::*;
        let n = 10;
        let r = nth_ugly_number(n);
        println!("{r}")
    }
}
