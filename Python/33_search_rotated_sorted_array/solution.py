from typing import List


def search(nums: List[int], target: int) -> int:
    """
    This function takes in a posibly rotated sorted list of
    distinct integers and a target integer and returns the index
    of the target interger if it is in the list,
    else it returns -1.

    Args:
    -  nums (List[int]): a list of integers -- potentially rotated
                        Guaranteed to be distinct and sorted
    -  target (int): an integer to search for in the list

    Returns:
    -  int: the index of the target integer if it is in the list,
            else -1

    Runtime:   O(log(n))
    Space:     O(1)

    Examples:
    >>> search([4,5,6,7,0,1,2], 0)
    4
    >>> search([4,5,6,7,0,1,2], 3)
    -1
    >>> search([1], 0)
    -1
    """

    if not nums:
        return -1

    low, high = 0, len(nums) - 1

    while low <= high:
        mid = (low + high) // 2
        if nums[mid] == target:
            return mid

        # if left side is sorted
        if nums[low] <= nums[mid]:
            if nums[low] <= target <= nums[mid]:
                high = mid - 1
            else:
                low = mid + 1
        else:
            if nums[mid] <= target <= nums[high]:
                low = mid + 1
            else:
                high = mid - 1
    return -1

# Tests


def test():
    assert search([4, 5, 6, 7, 0, 1, 2], 0) == 4
    assert search([4, 5, 6, 7, 0, 1, 2], 3) == -1
    assert search([1], 0) == -1
    assert search([], 3) == -1
    assert search([1, 3], 3) == 1
    assert search([3, 1], 3) == 0
    assert search([1], 1) == 0
    assert search([4, 5, 6, 7, 8, 9, 1, 2, 3], 3) == 8
    print("All tests passed.")


if __name__ == "__main__":
    test()
