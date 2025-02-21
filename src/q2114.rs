pub fn most_words_found(sentences: &[&str]) -> usize {
    sentences
        .iter()
        .map(|x| x.split(' ').count())
        .max()
        .unwrap_or_default()
}
