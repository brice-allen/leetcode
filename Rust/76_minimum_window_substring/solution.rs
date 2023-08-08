fn min_window(s: String, t: String) -> String {
    let mut t_freq = vec![0; 128];
    for char in t.bytes() {
        t_freq[char as usize] += 1;
    }

    let mut s_freq = vec![0; 128];
    let mut left = 0;
    let mut required = t.len();
    let mut ans = (usize::MAX, 0, 0); // length, left, right

    let s_bytes = s.as_bytes();
    for right in 0..s.len() {
        let r_char = s_bytes[right];
        s_freq[r_char as usize] += 1;

        if t_freq[r_char as usize] > 0 && s_freq[r_char as usize] <= t_freq[r_char as usize] {
            required -= 1;
        }

        while required == 0 {
            let l_char = s_bytes[left];
            if right - left < ans.0 {
                ans = (right - left, left, right);
            }
            s_freq[l_char as usize] -= 1;

            if t_freq[l_char as usize] > 0 && s_freq[l_char as usize] < t_freq[l_char as usize] {
                required += 1;
            }

            left += 1;
        }
    }

    if ans.0 == usize::MAX {
        String::new()
    } else {
        s.chars().skip(ans.1).take(ans.2 - ans.1 + 1).collect()
    }
}

fn main() {
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    println!("{}", min_window(s, t)); // Expected output: "BANC"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        assert_eq!(min_window(s, t), "BANC");
    }
}
