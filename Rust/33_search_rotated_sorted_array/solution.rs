/// Searches for a target value in a rotated sorted array and returns its index.
/// 
/// The function takes a vector 'nums' which is sorted in ascending order but
/// is possibly rotated at an unknown pivot, and an interger 'target'.  
/// The function returns the index of the target value if it is found in 'nums'
/// else it returns -1 if it is not found. 
/// The function uses a modified binary search algorithm to find the target value.
/// Runtime Complexity: O(log n). 
///
///
/// # Arguments
/// * nums - a vector of integers i32 -- guaranteed to be sorted in ascending order
/// * target - an integer i32 -- the value to be searched for in nums

/// # Returns
/// * i32 - the index of the target value if it is found in nums, else -1
///
///
/// # Examples
/// ```
/// assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
/// assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1); 
/// assert_eq!(search(vec![1], 0), -1);
/// '''

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid as usize] == target {
            return mid as i32;
        }
        if nums[left as usize] <= nums[mid as usize] {
            if target >= nums[left as usize] && target < nums[mid as usize] {
                right = mid -1;
            }else {
                left = mid + 1;
            }
        }else{
            if target > nums[mid as usize] && target <= nums[right as usize] {
                left = mid + 1;
            }else {
                right = mid - 1;
            }
        }
    }
    -1
}

fn main(){
    println!("Search in Rotated Sorted Array: {}", search(vec![4,5,6,7,0,1,2], 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
        assert_eq!(search(vec![1], 0), -1);
    }
}