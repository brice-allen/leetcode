fn minimum_delete_sum(s1: &str, s2: &str) -> i32 {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    let len1 = s1_bytes.len();
    let len2 = s2_bytes.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in (0..len1).rev() {
        dp[i][len2] = dp[i + 1][len2] + (s1_bytes[i] as i32);
    }

    for i in (0..len2).rev() {
        dp[len1][i] = dp[len1][i + 1] + (s2_bytes[i] as i32);
    }

    for i in (0..len1).rev {
        for j in (0..len2).rev() {
            if s1_bytes[i] == s2_bytes[j] {
                dp[i][j] = dp[i + 1][j + 1];
            } else {
                dp[i][j] = std::cmp::min(
                    dp[i + 1][j] + (s1_bytes[i] as i32),
                    dp[i][j + 1] + (s2_bytes[j] as i32)
                );
            }
        }
    }
    dp[0][0]
}
