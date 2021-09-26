macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (s, t, expected) = $value;
            assert_eq!(expected, valid_anagram(s, t));
        }
    )*
    }
}

tests! {
    ex1: ("tas".to_string(), "sat".to_string(), true),
    ex2: ("rat".to_string(), "cat".to_string(), false),
    ex3: ("anagram".to_string(), "nagaram".to_string(), true),
}

use std::collections::HashMap;

pub fn valid_anagram(s: String, t: String) -> bool {
    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();

    for c in s.bytes() {
        *s_map.entry(c).or_insert(0) += 1;
    }

    for c in t.bytes() {
        *t_map.entry(c).or_insert(0) += 1;
    }

    s_map == t_map
}

fn main() {
    println!("{:?}", valid_anagram("tas".to_string(), "sat".to_string()));
    println!("{:?}", valid_anagram("rat".to_string(), "cat".to_string()));
    println!(
        "{:?}",
        valid_anagram("anagram".to_string(), "nagaram".to_string())
    );
}
