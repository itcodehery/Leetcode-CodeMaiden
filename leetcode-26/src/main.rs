fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut res_vec: Vec<i32> = Vec::new();
    res_vec.push(*nums.iter().nth(0).unwrap());
    for i in 0..nums.len() {
        let nth: &i32 = nums.iter().nth(i).unwrap();
        if res_vec.contains(&nth) {
            continue;
        }
        res_vec.push(*nums.iter().nth(i).unwrap());
    }
    *nums = res_vec.clone();
    res_vec.len() as i32
}

fn main() {
    let mut inp_vec: Vec<i32> = vec![10, 11, 22, 22, 23, 24, 25, 25, 100, 100, 200];
    println!("{}", remove_duplicates(&mut inp_vec));
    println!("{:?}", inp_vec);
}
