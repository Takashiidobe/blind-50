macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, valid_parentheses(input));
        }
    )*
    }
}

tests! {
    ex1: ("()".to_string(), true),
    ex2: ("()[]{}".to_string(), true),
    ex3: ("(]".to_string(), false),
    ex4: ("([)]".to_string(), false),
    ex5: ("{[]}".to_string(), true),
    ex6: ("{".to_string(), false),
}

pub fn valid_parentheses(s: String) -> bool {
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

fn main() {
    println!("{:?}", valid_parentheses("()".to_string()));
    println!("{:?}", valid_parentheses("()[]{}".to_string()));
    println!("{:?}", valid_parentheses("(]".to_string()));
    println!("{:?}", valid_parentheses("([)]".to_string()));
    println!("{:?}", valid_parentheses("{[]}".to_string()));
    println!("{:?}", valid_parentheses("{".to_string()));
}
