pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut i, mut j, mut k) = (m as usize, n as usize, (m + n) as usize);
    while j > 0 {
        if i > 0 && nums1[i - 1] > nums2[j - 1] {
            nums1[k - 1] = nums1[i - 1];
            i -= 1;
        } else {
            nums1[k - 1] = nums2[j - 1];
            j -= 1;
        }
        k -= 1;
    }
}

fn main() {
    let mut inp_vec: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let mut inp_vec2: Vec<i32> = vec![2, 5, 6];

    merge(&mut inp_vec, 3, &mut inp_vec2, 3);
    println!("{:?}", inp_vec);
}
