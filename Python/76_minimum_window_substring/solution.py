def min_window(s: str, t: str) -> str:
    from collections import Counter, defaultdict

    # If either string is empty, return an empty string
    if not (s and t):
        return ""

    # Dictionary to keep the count of all characters in t
    dict_t = Counter(t)

    # Number of unique characters in t,
    # required to be present in the desired window.
    required = len(dict_t)

    # Left and right pointer
    l, r = 0, 0

    # Dictionary to keep the count of characters in the current window
    window_counts = defaultdict(int)

    # Number of unique characters from t in the current window
    formed = 0

    # Tuple to keep the results (length, left, right)
    ans = float("inf"), None, None

    while r < len(s):
        character = s[r]

        # Add the character to the window_counts
        window_counts[character] += 1

        # If the frequency of current character matches with
        # desired count in t,
        # increment formed
        if character in dict_t:
            if window_counts[character] == dict_t[character]:
                formed += 1

        # Contract the window until the condition is no longer valid
        while l <= r and formed == required:
            character = s[l]

            # Update the results
            if r - l + 1 < ans[0]:
                ans = (r - l + 1, l, r)

            window_counts[character] -= 1
            if character in dict_t:
                if window_counts[character] < dict_t[character]:
                    formed -= 1

            l += 1

        # Expand the window
        r += 1

    return "" if ans[0] == float("inf") else s[ans[1]: ans[2] + 1]


# Test cases
s = "ADOBECODEBANC"
t = "ABC"
print(min_window(s, t))  # Expected: "BANC"
