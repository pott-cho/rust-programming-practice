pub fn sort_sentence(s: &str) -> Result<String, String> {
    if s.len() < 2 {
        return Err(String::from("문자열의 길이가 최소단위 이하입니다."));
    }

    // 1. 문장을 단어로 분리한다.
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut arr: Vec<(&str, &str)> = words
        .iter()
        .map(|x| (&x[..x.len() - 1], &x[x.len() - 1..]))
        .collect();

    // 2. 각 단어의 마지막 글자를 인덱스로 활용하여 단어를 정렬한다.
    arr.sort_unstable_by_key(|&(_, index)| index);

    // 3. 배열을 풀어서 String 으로 만들어 반환한다.
    Ok(arr
        .iter()
        .map(|(word, _)| *word)
        .collect::<Vec<&str>>()
        .join(" "))
}
