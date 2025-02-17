use std::collections::HashMap;

pub fn num_identical_pairs(nums: &[i32]) -> i32 {
    let mut result = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    // 현재 값을 해시맵에서 찾는다. 있으면 +1, 없으면 entry 1 등록
    for n in nums {
        map.entry(*n)
            .and_modify(|e| {
                result += *e;
                *e += 1;
            })
            .or_insert(1);
    }
    result
}
