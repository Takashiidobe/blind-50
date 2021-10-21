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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(valid_parentheses("()"), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(valid_parentheses("()[]{}"), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(valid_parentheses("(]"), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(valid_parentheses("([)]"), false);
    }

    #[test]
    fn test_5() {
        assert_eq!(valid_parentheses("{[]}"), true);
    }

    #[test]
    fn test_6() {
        assert_eq!(valid_parentheses("{"), false);
    }
}
