/// Returns the probability that a knight remains on the board after `k` moves.
///
/// # Arguments
/// * `n` - The size of the chessboard (n x n).
/// * `k` - The number of moves the knight attempts to make.
/// * `row` - The starting row of the knight.
/// * `column` - The starting column of the knight.
///
/// # Returns
/// * A `f64` representing the probability that the knight remains on the board after `k` moves.
pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let moves = [
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
    ];
    let n = n as usize;
    let mut dp = vec![vec![vec![0.0; n]; n]; k as usize + 1];
    dp[0][row as usize][column as usize] = 1.0;

    for step in 1..=k {
        for r in 0..n {
            for c in 0..n {
                for &(dr, dc) in moves.iter() {
                    let prev_r = (r as i32) + dr;
                    let prev_c = (c as i32) + dc;
                    if prev_r >= 0 && prev_r < (n as i32) && prev_c >= 0 && prev_c < (n as i32) {
                        dp[step as usize][r][c] +=
                            dp[(step - 1) as usize][prev_r as usize][prev_c as usize] / 8.0;
                    }
                }
            }
        }
    }

    dp[k as usize]
        .iter()
        .map(|row| row.iter().sum::<f64>())
        .sum()
}

fn main() {
    let n = 3;
    let k = 2;
    let row = 0;
    let column = 0;
    let probability = knight_probability(n, k, row, column);
    println!("The probability of the knight remaining on the board is: {}", probability);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_probability() {
        let n = 3;
        let k = 2;
        let row = 0;
        let column = 0;
        let expected = 0.0625;
        assert!((knight_probability(n, k, row, column) - expected).abs() < 1e-6);
    }
}
