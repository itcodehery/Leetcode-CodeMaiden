// Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.
//
// Example 1:
//
// Input: temperatures = [73,74,75,71,69,72,76,73]
// Output: [1,1,4,2,1,1,0,0]

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![0; temperatures.len()];

    if temperatures.is_empty() {
        return answer;
    }

    for i in 0..temperatures.len() {
        let initial_val = temperatures[i];
        println!("\nIteration with initial_val {}", initial_val);
        let mut counter = 0;

        for j in i..temperatures.len() {
            if temperatures[j] < initial_val || initial_val == temperatures[j] {
                // the next value is smaller than the main value of current iteration
                println!("Pushing {} to stack", temperatures[j]);
                counter += 1;
            } else {
                // the next value is larger than the main value of cur ite
                println!("Final Counter: {}", counter);
                answer[i] = counter;
                println!("{}", answer[i]);
                break;
            }
        }
    }

    answer
}

fn main() {
    println!(
        "{:?}",
        daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
    );
}
