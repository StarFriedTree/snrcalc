
pub fn absolute_sum_of_int_list (list: &[i32]) -> u32 {
   list.iter().map(|&num| num.abs() as u32).sum()
}

pub fn add_two_ints (a: i32, b: i32) -> i32 {
    a + b
}

