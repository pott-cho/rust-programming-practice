pub fn most_words_found(sentences: Vec<&str>) -> usize {
    let mut results = 0;
    for sentence in sentences {
        let count = sentence.split(' ').count();
        if count > results {
            results = count;
        }
    }
    results
}
