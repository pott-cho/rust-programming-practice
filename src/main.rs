mod q1920;
mod q1929;

fn main() {
    let result = q1929::get_concatenation(&[1, 2, 1]);
    assert_eq!(result, vec![1, 2, 1, 1, 2, 1]);

    let result = q1920::build_array(&[0, 2, 1, 5, 3, 4]);
    assert_eq!(result, [0, 1, 2, 4, 5, 3]);
}
