pub fn number_of_matches(n: i32) -> i32 {
    let mut matches = 0;
    let mut teams = n;
    while teams > 1 {
        if teams % 2 == 0 {
            teams /= 2;
            matches += teams;
        } else {
            teams /= 2;
            matches += teams;
            teams += 1;
        }
    }
    matches
}
