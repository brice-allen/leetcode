  

fn binary_search(nums: &Vec<i32>, target: i32, first: bool) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    let mut mid;

    while left < right {
        mid = left + (right - left) / 2;

        if nums[mid] > target || (first && nums[mid] == target) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left as i32
}
 pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![-1, -1];

    let left_idx = Solution::binary_search(&nums, target, true);

    if left_idx == nums.len() as i32 || nums[left_idx as usize] != target {
        return result;
    }

    result[0] = left_idx;
    result[1] = Solution::binary_search(&nums, target, false) - 1;

    result
}