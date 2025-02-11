pub fn decode_message(key: &str, message: &str) -> String {
    let mut code = [' '; 26];
    let first = b'a';
    let mut alpha = 0;
    for c in key.chars() {
        if c == ' ' {
            continue;
        }

        let idx = (c as u8 - (b'a')) as usize;
        if code[idx] == ' ' {
            code[idx] = (alpha + first) as char;
            alpha += 1;
        }
    }

    let mut result = String::new();
    for m in message.chars() {
        if m == ' ' {
            result.push(' ');
        } else {
            let idx: usize = (m as u8 - first) as usize;
            result.push(code[idx]);
        }
    }

    result
}
