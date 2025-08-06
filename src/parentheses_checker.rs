use crate::stack::Stack;
use std::collections::HashMap;
// TODO: use custom hashmap

fn validate_parentheses(paren: &str) -> bool {
    if paren.is_empty() {
        return true;
    }
    if paren.len() % 2 != 0 {
        return false;
    }

    let paren_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    let mut stack = Stack::new();
    for ch in paren.chars() {
        if paren_map.contains_key(&ch) {
            stack.push(ch);
            continue;
        }
        match stack.pop() {
            Some(top) => {
                if *paren_map.get(&top).unwrap() != ch {
                    return false;
                }
            }
            None => return false,
        }
    }
    stack.is_empty()
}

///////////////////////
#[test]
fn test_balanced_parentheses() {
    let cases = Vec::from([
        ("", true),
        ("()(())", true),
        ("()[]{}", true),
        ("{[([])]}", true),
        ("(", false),
        ("()(()", false),
        ("(]{)[[}", false),
        ("])}[", false),
    ]);

    for case in cases {
        assert_eq!(validate_parentheses(case.0), case.1);
    }
}
