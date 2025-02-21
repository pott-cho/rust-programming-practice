mod q1108;
mod q1281;
mod q1290;
mod q1313;
mod q1342;
mod q1365;
mod q1389;
mod q1431;
mod q1470;
mod q1480;
mod q1512;
mod q1528;
mod q1672;
mod q1688;
mod q1720;
mod q1773;
mod q1859;
mod q1920;
mod q1929;
mod q2011;
mod q2037;
mod q2114;
mod q2160;
mod q2235;
mod q2325;
mod q771;

fn main() {
    let result = q2235::sum(12, 5);
    assert_eq!(result, 17);

    let result = q1929::get_concatenation(&[1, 2, 1]);
    assert_eq!(result, vec![1, 2, 1, 1, 2, 1]);

    let result = q1920::build_array(&[0, 2, 1, 5, 3, 4]);
    assert_eq!(result, [0, 1, 2, 4, 5, 3]);

    let result = q1480::running_sum(&[1, 2, 3, 4]);
    assert_eq!(result, Vec::from([1, 3, 6, 10]));

    let result = q1672::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]);
    assert_eq!(result, 6);

    let result = q1470::shuffle(&[2, 5, 1, 3, 4, 7], 3);
    assert_eq!(result, vec![2, 3, 5, 4, 1, 7]);

    let result = q2160::minimum_sum(2932);
    assert_eq!(result, 52);
    let result = q2160::minimum_sum(4009);
    assert_eq!(result, 13);

    let result = q1512::num_identical_pairs(&[1, 2, 3, 1, 1, 3]);
    assert_eq!(result, 4);
    let result = q1512::num_identical_pairs(&[1, 1, 1, 1]);
    assert_eq!(result, 6);
    let result = q1512::num_identical_pairs(&[1, 2, 3]);
    assert_eq!(result, 0);

    let result = q1431::kids_with_candies(&[2, 3, 5, 1, 3], 3);
    assert_eq!(result, vec![true, true, true, false, true]);

    let result = q1281::subtract_product_and_sum(234);
    assert_eq!(result, 15);

    let result = q1365::smaller_numbers_than_current(&[8, 1, 2, 2, 3]);
    assert_eq!(result, vec![4, 0, 1, 1, 3]);
    let result = q1365::smaller_numbers_than_current(&[7, 7, 7, 7]);
    assert_eq!(result, vec![0, 0, 0, 0]);

    let result = q1108::defang_i_paddr("1.1.1.1");
    assert_eq!(result, String::from("1[.]1[.]1[.]1"));

    let result = q771::num_jewels_in_stones("aA", "aAAbbbb");
    assert_eq!(result, Ok(3));

    let result = q2114::most_words_found(vec![
        "alice and bob love leetcode",
        "i think so too",
        "this is great thanks very much",
    ]);
    assert_eq!(result, 6);

    let result = q1859::sort_sentence("is2 sentence4 This1 a3");
    assert_eq!(result, String::from("This is a sentence"));

    let result = q1773::count_matches(
        &[
            vec!["phone", "blue", "pixel"],
            vec!["computer", "silver", "lenovo"],
            vec!["phone", "gold", "iphone"],
        ],
        "color",
        "silver",
    );
    assert_eq!(result, 1);

    let result = q1720::decode(&[1, 2, 3], 1);
    assert_eq!(result, vec![1, 0, 2, 1]);

    let result = q1313::decompress_rl_elist(&[1, 2, 3, 4]);
    assert_eq!(result, [2, 4, 4, 4]);

    let result = q1528::restore_string("codeleet", &[4, 5, 6, 7, 0, 2, 1, 3]);
    assert_eq!(result, "leetcode");

    let result = q2325::decode_message(
        "the quick brown fox jumps over the lazy dog",
        "vkbs bs t suepuv",
    );
    assert_eq!(result, "this is a secret");

    let linked_list = q1290::Node::make_singly_linked_list(&[1, 0, 1]);
    let result = q1290::get_decimal_value(linked_list);
    assert_eq!(result, 5);

    let result = q2011::final_value_after_operations(vec!["--X", "X++", "X++"]);
    assert_eq!(result, 1);

    let result = q1342::number_of_steps(14);
    assert_eq!(result, 6);

    let result = q1389::create_target_array(&[0, 1, 2, 3, 4], &[0, 1, 2, 2, 1]);
    assert_eq!(result, vec![0, 4, 1, 3, 2]);

    let result = q1688::number_of_matches(7);
    assert_eq!(result, 6);

    let result = q2037::min_moves_to_seat(&[3, 1, 5], &[2, 7, 4]);
    assert_eq!(result, 4);
}
