pub fn min_moves_to_seat(seats: &[i32], students: &[i32]) -> i32 {
    let mut seats = seats.to_vec();
    let mut students = students.to_vec();

    seats.sort_unstable();
    students.sort_unstable();

    let mut total_moves = 0;
    for i in 0..seats.len() {
        total_moves += (seats[i] - students[i]).abs();
    }

    total_moves
}
