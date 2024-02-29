fn main() {
    println!("Hello, world!");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            let sum = sorted_nums[i] + sorted_nums[j];
            if sum > target {
                break;
            }
            if sum == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    panic!("not found");
}

#[test]
fn test() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
