
pub fn absolute_sum_of_int_list (list: &[i32]) -> u32 {
   list.iter().map(|&num| num.abs() as u32).sum()
}

pub fn add_two_ints (a: i32, b: i32) -> i32 {
    a + b
}

pub fn more_odd_in_list (list: &[i32]) -> bool {
    let result: i32 = list.iter().map(|&num| if num % 2 == 1 {1} else {-1}).sum();
    result > 0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn more_odd() {
        let positive = more_odd_in_list (&[1, 2, 3, 4, 5]);
        let negative = more_odd_in_list (&[1, 2, 3, 4]);
        assert_eq!(positive, true);
        assert_eq!(negative, false);
    }
}