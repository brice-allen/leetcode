#include <iostream>
#include <string>
#include <unordered_map>
#include <climits>
#include <cassert>

using namespace std;

string min_window(string s, string t) {
    unordered_map<char, int> t_freq, s_freq;
    for (char c : t) {
        t_freq[c]++;
    }

    int left = 0, right = 0, required = t.size(), min_len = INT_MAX, min_start = 0;

    while (right < s.size()) {
        char r_char = s[right];
        if (t_freq.find(r_char) != t_freq.end()) {
            s_freq[r_char]++;
            if (s_freq[r_char] <= t_freq[r_char]) {
                required--;
            }
        }

        while (required == 0) {
            if (right - left < min_len) {
                min_len = right - left;
                min_start = left;
            }

            char l_char = s[left];
            if (t_freq.find(l_char) != t_freq.end()) {
                s_freq[l_char]--;
                if (s_freq[l_char] < t_freq[l_char]) {
                    required++;
                }
            }
            left++;
        }
        right++;
    }

    return min_len == INT_MAX ? "" : s.substr(min_start, min_len + 1);
}

int main() {
    string s = "ADOBECODEBANC";
    string t = "ABC";
    assert(min_window(s, t) == "BANC");
    cout << min_window(s, t) << endl;  // Expected output: "BANC"
}

