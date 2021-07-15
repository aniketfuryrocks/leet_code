/// [Find Peak Element](https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3812/)
#[allow(dead_code)]
fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut x = 0;
    let mut highest = i32::MIN;
    let mut highest_index = 0;

    let len = nums.len();

    while x < len {
        let left = if x == 0 { i32::MIN } else { nums[x - 1] };
        let mid = nums[x];
        if mid > left {
            if x + 1 == len { 
                if mid > highest {
                    return x as i32;
                } else {
                    return highest_index;
                }
            }
            
            let right = nums[x + 1];

            if mid > right {
                highest_index = x as i32;
                highest = mid;
            }
        }
        x+=1;
    }

    return highest_index;
}

#[test]
fn find_peak_element_test() {
    assert_eq!(find_peak_element(vec![1,2,1,3,5,6,4]), 5);
    assert_eq!(find_peak_element(vec![2,3,4]), 2);
}
