/// Function `length_of_lis` finds the length of the longest increasing subsequence in a vector of integers.
///
/// # Arguments
///
/// * `nums` - A vector of integers. It is not required to be sorted.
///
/// # Returns
///
/// * An `i32` that is the length of the longest strictly increasing subsequence in `nums`.
///
/// # Example
///
/// ```
/// let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
/// let result = length_of_lis(nums);
/// assert_eq!(result, 4); // The longest increasing subsequence is [2, 5, 7, 101]
/// ```
///
/// # Complexity
///
/// This function runs in O(n log n) time, where n is the length of `nums`. 
/// It uses dynamic programming with binary search to improve upon the naive O(n^2) dynamic programming solution.
/// Each element in `nums` is handled exactly once, and for each element, a binary search is performed on an auxiliary array.
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![];
    for &num in nums.iter() {
        match dp.binary_search(&num) {
            Ok(_) => (),
            Err(pos) => {
                if pos == dp.len() {
                    dp.push(num);
                } else {
                    dp[pos] = num;
                }
            }
        }
    }
    dp.len() as i32
}


fn main() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let result = length_of_lis(nums);
    println!("The length of the longest increasing subsequence is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(length_of_lis(nums), 4);
    }
}