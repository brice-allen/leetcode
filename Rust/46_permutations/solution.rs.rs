impl Solution{
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty(){
            return vec![vec![]]
        }
        let mut result = Vec.new();

        for (i, &num) in nums.iter().enumerate() {
            let mut nums_remaining = nums.clone();
            nums_remaining.remove(i);

            let mut sub_result = Solution::permute(nums_remaining);

            for perm in sub result.iter_mut(){
                perm.push(num);
            }
            result.append(&mut sub_result);
        }
        result

    }
}
