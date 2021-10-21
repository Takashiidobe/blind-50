#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(valid_anagram("tas", "sat"), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(valid_anagram("rat", "cat"), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(valid_anagram("anagram", "nagaram"), true);
    }
}

use std::collections::HashMap;

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
