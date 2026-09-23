impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for bracket in s.chars() {
            match bracket {
                '(' | '{' | '[' => stack.push(bracket),

                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }

                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }

                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }

                _ => return false,
            }
        }

        stack.is_empty()
    }
}