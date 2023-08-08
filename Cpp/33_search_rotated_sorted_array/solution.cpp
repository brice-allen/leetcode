#include <vector>
#include <iostream>
#include <cassert>
#include <numeric>

/**
* @brief: Searches for a target value in a rotated sorted array
* @param: std::vector<int>& nums - a vector of integers --
*         Guaranteed to be in ascending order, but rotated at some pivot
*
* @param: int target - the target value to search for
* @return: int - the index of the target value if found, else -1
*
* This function takes a vector 'nums' which is guaranteed to be in ascending order,
* but rotated at some pivot. It returns the index of the target value if found, else -1.
* This function uses a modified binary search algorithm.
* Runtime: O(log n) -- The reason for this is that the solution utilizes a binary search approach.
* At each iteration of the while loop, the search range is halved, making the worst-case scenario
* logarithmic to the size of the input array.
* Space: O(1) -- This is because the solution only uses a constant amount of extra space,
* irrespective of the size of the input.
* Specifically, space is used for variables left, right, and mid,
* but no additional data structures (like arrays or lists)
* that would scale with the size of the input are used.
*/


int search(const std::vector<int>& nums, int target) {
    std::size_t left = 0;
    std::size_t right;
    right = nums.size() - 1;

    while (left <= right) {
        //std::size_t mid = std::midpoint(right,left);
        std::size_t mid = left + (right - left) / 2;

        if (nums[mid] == target) {
            return mid;
        }

        if (nums[left] <= nums[mid]) {
            target >= nums[left] && target < nums[mid] ? right = mid - 1 : left = mid + 1;
        } else target > nums[mid] && target <= nums[right] ? left = mid + 1 : right = mid - 1;

    }

    return -1;
}

int main() {
    std::vector<int> nums = { 4,5,6,7,0,1,2 };
    std::cout << "Searching for 0 in { 4,5,6,7,0,1,2 }"
              << std::endl << "Expected: 4" << std::endl << "Actual: " << search(nums, 0) << std::endl;
    assert(search(nums, 0) == 4);
    assert(search(nums, 3) == -1);
    std::cout << "All tests passed." << std::endl;
}