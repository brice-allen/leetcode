#include <iostream>
#include <vector>
#include <unordered_map>

/*
 * Function: two_sum
 * ------------------
 * Given an array of integers "nums" and an integer "target", this function returns
 * indices of the two numbers such that they add up to target.
 *
 * @param nums: Vector of integers.
 * @param target: The target sum.
 * @return: A vector containing the two indices of the numbers which sum up to the target.
 * 
 * Time Complexity: O(n), where n is the size of the vector nums.
 * Space Complexity: O(n), due to the usage of unordered_map.
 */

std::vector<int> two_sum(const std::vector<int>& nums, int target) {
    std::unordered_map<int, int> indices_map;
    for (int i = 0; i < nums.size(); ++i) {
        int complement = target - nums[i];
        if (indices_map.find(complement) != indices_map.end()) {
            return {indices_map[complement], i};
        }
        indices_map[nums[i]] = i;
    }
    return {}; // Return empty vector if no solution (though the problem guarantees a solution).
}

int main() {
    // Test the function
    std::vector<int> nums = {2, 7, 11, 15};
    int target = 9;
    std::vector<int> result = two_sum(nums, target);
    std::cout << "Indices: [" << result[0] << ", " << result[1] << "]" << std::endl;

    // Additional test
    nums = {3, 2, 4};
    target = 6;
    result = two_sum(nums, target);
    std::cout << "Indices: [" << result[0] << ", " << result[1] << "]" << std::endl;

    return 0;
}
