// Generate Parentheses
// 
// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        fn permute(result: &mut Vec<String>, mut parenthesis: String, left: i32, right: i32, n: i32) {
            if parenthesis.len() == (2 * n) as usize {
                result.push(parenthesis);
                return;
            }
            if left < n {
                parenthesis.push('(');
                permute(result, parenthesis.clone(), left+1, right, n);
                parenthesis.pop();
            }
            if right < left {
                parenthesis.push(')');
                permute(result, parenthesis.clone(), left, right + 1, n);
                parenthesis.pop();
            }
        }
        permute(&mut result, String::new(), 0, 0, n);
        return result;
    }
}
