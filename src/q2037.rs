pub fn min_moves_to_seat(seats: &mut [i32], students: &mut [i32]) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();

    (0..seats.len())
        .map(|x| (seats[x] - students[x]).abs())
        .sum()
}
