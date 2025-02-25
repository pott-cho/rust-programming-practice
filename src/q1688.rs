pub fn number_of_matches(n: i32) -> i32 {
    let mut matches = 0;
    let mut teams = n;
    while teams > 1 {
        matches += teams / 2;
        teams = (teams / 2) + (teams % 2);
    }
    matches
}
