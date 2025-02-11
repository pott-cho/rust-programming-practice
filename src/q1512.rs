pub fn num_identical_pairs(nums: &[i32]) -> i32 {
    let size = nums.len();
    let mut result = 0;

    // n제한이 100이기 때문에 간단하게 O(n^2) 으로 풀 수 있다.
    for i in 0..size {
        for j in i+1..size {
            if nums[i] == nums[j] {
                result+=1;
            }
        }
    }

    result
}
