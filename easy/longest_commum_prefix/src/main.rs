impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let mut pre = strs[0].as_str();

        for string in &strs[1..] {
            while !string.starts_with(pre) {
                pre = &pre[0..pre.len() - 1];
                if pre.is_empty() {
                    return "".to_string();
                }
            }
        }

        pre.to_string()
    }
}
