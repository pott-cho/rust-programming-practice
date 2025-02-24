use std::collections::HashMap;

pub fn count_matches(items: &[Vec<&str>], rule_key: &str, rule_value: &str) -> Result<i32, String> {
    let map: HashMap<&str, usize> = HashMap::from([("type", 0), ("color", 1), ("value", 2)]);

    let target_index = match map.get(rule_key) {
        Some(key) => *key,
        None => return Err(String::from("Please check rule_key.")),
    };

    let result = items
        .iter()
        .filter(|x| x.get(target_index) == Some(&rule_value))
        .count();

    result
        .try_into()
        .map_err(|_| String::from("items size exceeds i32 range."))
}
