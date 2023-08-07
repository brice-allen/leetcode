// dynamic programming solution, slow
impl Solution{
	pub fn jump(nums: Vec<i32> -> i32{
	let n = nums.len();
	let mut dp = vec![std::i32::MAX; n];
	dp[0] = 0;
	
	for i in 0..n {
		for j in 1..=nums[i] as usize{
			if i+j < n {
				dp[i+j] = dp[i+j].min(dp[i] + 1)
			}	
		}	
	}
	dp[n-1]
}
}
