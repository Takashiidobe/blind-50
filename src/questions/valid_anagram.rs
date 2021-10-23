use crate::*;
use std::collections::HashMap;

test! {
    test_1: valid_anagram("tas", "sat"), true,
    test_2: valid_anagram("rat", "sat"), false,
    test_3: valid_anagram("", ""), true,
    test_4: valid_anagram("anagram", "nagaram"), true,
}

pub fn valid_anagram(s: &str, t: &str) -> bool {
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
