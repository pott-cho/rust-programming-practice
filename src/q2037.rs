pub fn min_moves_to_seat(seats: &mut [i32], students: &mut [i32]) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();

    seats
        .iter()
        .zip(students.iter())
        .map(|(s1, s2)| (s1 - s2).abs())
        .sum()
}
