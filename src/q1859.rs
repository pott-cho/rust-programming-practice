pub fn sort_sentence(s: &str) -> String {
    // 1. 문장을 단어로 분리한다.
    let words: Vec<&str> = s.split_whitespace().collect();
    let size = words.len();
    let mut arr = [""; 201];

    // 2. 각 단어의 마지막 글자를 인덱스로 활용하여 단어를 배열에 넣는다.
    for word in &words {
        let index = word.chars().nth_back(0).unwrap();
        let index: usize = index as usize - 0x30;
        arr[index] = &word[0..word.len() - 1];
    }

    // 3. 배열을 풀어서 String 으로 만들어 반환한다.
    arr[1..=size].join(" ")
}
