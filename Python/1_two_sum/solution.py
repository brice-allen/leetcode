from typing import List, Dict

def two_sum(nums: List[int], target: int) -> List[int]:
    """
    Given an array of integers "nums" and an integer "target", this function returns
    indices of the two numbers such that they add up to target.

    Args:
    - nums (List[int]): List of integers.
    - target (int): The target sum.

    Returns:
    - List[int]: A list containing the two indices of the numbers which sum up to the target.

    Time Complexity: O(n), where n is the size of the list nums.
    Space Complexity: O(n), due to the usage of a dictionary.
    """
    
    indices_map: Dict[int, int] = {}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in indices_map:
            return [indices_map[complement], i]
        indices_map[num] = i
    return []

def main():
    # Test the function
    nums = [2, 7, 11, 15]
    target = 9
    result = two_sum(nums, target)
    print(f"Indices: {result}")

    # Additional test
    nums = [3, 2, 4]
    target = 6
    result = two_sum(nums, target)
    print(f"Indices: {result}")

if __name__ == "__main__":
    main()
