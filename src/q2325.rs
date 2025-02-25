use std::collections::{hash_map::Entry, HashMap};

pub fn decode_message(key: &str, message: &str) -> String {
    let first = b'a';
    let mut alpha = 0;
    let mut code: HashMap<char, char> = HashMap::new();

    // 1. 키 매핑 테이블 생성
    for c in key.chars() {
        if c == ' ' {
            continue;
        }

        if let Entry::Vacant(e) = code.entry(c) {
            e.insert((first + alpha) as char);
            alpha += 1;
        }
    }

    // 2. 메시지 디코딩
    message
        .chars()
        .map(|c| *code.get(&c).unwrap_or(&c))
        .collect()
}
