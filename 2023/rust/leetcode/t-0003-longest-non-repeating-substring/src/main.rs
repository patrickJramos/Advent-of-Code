fn main() {
    println!("Hello, world!");
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut biggest = 0;

    let mut i = 0;
    while i + biggest < s.len() {
        let slice = &s[i..i + biggest + 1];
        if !has_duplicates(slice) {
            biggest += 1;
        } else {
            i += 1;
        }
    }

    biggest as i32
}

fn has_duplicates(str: &str) -> bool {
    let mut found_strings = vec![];
    for c in str.chars() {
        if found_strings.contains(&c) {
            return true;
        }
        found_strings.push(c);
    }
    false
}

#[test]
fn test() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring(" ".to_string()), 1);
}
