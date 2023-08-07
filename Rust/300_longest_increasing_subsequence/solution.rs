/// Calculates the length of the longest increasing subsequence in a given vector of integers.
///
/// # Arguments
///
/// * `nums` - A vector of i32 integers. This vector is not required to be sorted.
///
/// # Returns
///
/// * An i32 integer representing the length of the longest strictly increasing subsequence within `nums`.
///
/// # Example
///
/// ```
/// let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
/// let result = length_of_lis(nums);
/// assert_eq!(result, 4); // The longest increasing subsequence is [5, 3, 7, 101].
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Safety
///
/// This function does not use any unsafe code.
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }   
    *dp.iter().max().unwrap()
}

// Here's the `main` function:

fn main() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let result = length_of_lis(nums);
    println!("The length of the longest increasing subsequence is: {}", result);
}

// And here's a test:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(length_of_lis(nums), 4);
    }
}