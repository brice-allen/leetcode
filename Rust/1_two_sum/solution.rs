/// Given an array of integers `nums` and an integer `target`,
/// returns the indices of the two numbers such that they add up to target.
///
/// # Arguments
/// * `nums` - A vector of integers.
/// * `target` - The target sum.
///
/// # Returns
/// A tuple of two usize values representing the indices of the numbers in `nums` 
/// that add up to `target`.
///
/// # Examples
/// ```
/// let nums = vec![2, 7, 11, 15];
/// let target = 9;
/// assert_eq!(two_sum(nums, target), (0, 1));
/// ```
pub fn two_sum(nums: Vec<i32>, target: i32) -> (usize, usize) {
    use std::collections::HashMap;

    let mut indices_map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_idx) = indices_map.get(&complement) {
            return (complement_idx, i);
        }
        indices_map.insert(num, i);
    }
    
    panic!("No solution found!"); // This should not be reached as per problem statement
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert!(result == (0, 1) || result == (1, 0));
    }
}

fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = two_sum(nums, target);
    println!("Indices: {:?}", result);
}
