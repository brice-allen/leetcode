/// Function `find_peak_element` finds the index of a peak element in a vector of integers.
///
/// A peak element is an element that is strictly greater than its neighbors.
///
/// # Arguments
///
/// * `nums` - A vector of integers. It is not required to be sorted.
///
/// # Returns
///
/// * An `usize` that is the index of a peak element in `nums`. 
///   If there are multiple peaks, it returns the index of any one of them.
///
/// # Example
///
/// ```
/// let nums = vec![1, 2, 3, 1];
/// let result = find_peak_element(nums);
/// assert_eq!(result, 2); // 3 is a peak element
/// ```
///
/// # Complexity
///
/// This function runs in O(log n) time, where n is the length of `nums`. 
/// It uses a binary search approach to find a peak element.
/// Each element in `nums` is handled exactly once, and for each element, a binary search is performed.
// return i32 when submitting to LeetCode
pub fn find_peak_element(nums: Vec<i32>) -> usize  {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[mid + 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    left
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let result = find_peak_element(nums);
    println!("The index of a peak element is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_peak_element() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(find_peak_element(nums), 2);
    }
}
