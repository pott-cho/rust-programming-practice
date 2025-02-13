pub fn count_matches(items: &[Vec<&str>], rule_key: &str, rule_value: &str) -> i32 {
    let mut result = 0;
    for item in items {
        let t = item[0];
        let color = item[1];
        let name = item[2];

        if (rule_key == "type" && rule_value == t)
            || (rule_key == "color" && rule_value == color)
            || (rule_key == "name" && rule_value == name)
        {
            result += 1;
        }
    }

    result
}
