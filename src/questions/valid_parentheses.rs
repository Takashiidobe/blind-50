use crate::*;

pub fn valid_parentheses(s: &str) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => unimplemented!(),
        }
    }
    stack.is_empty()
}

test! {
    test_1: valid_parentheses("()"), true,
    test_2: valid_parentheses("()[]{}"), true,
    test_3: valid_parentheses("(}"), false,
    test_4: valid_parentheses("([)]"), false,
    test_5: valid_parentheses("{[]}"), true,
    test_6: valid_parentheses("{[]}}"), false,
}
