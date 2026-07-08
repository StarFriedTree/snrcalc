use std::cmp::Ordering;

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

pub fn last_digit_c_equals_of_ab (a: i32, b: i32, c: i32) -> bool {
    let prod = (a * b) % 10;
    let c = c % 10;
    prod == c || prod + c == 0
}

pub fn count_distinct_quadratic_roots (a: i32, b: i32, c: i32) -> u8 {
    let bsq = b * b;
    let ac = 4 * a * c;
    match bsq.cmp(&ac) {
        Ordering::Less => 0u8,
        Ordering::Equal => 1u8,
        Ordering::Greater => 2u8,
    }
}

pub fn positive_descending_pair (a: u32, b: u32) -> &'static str {
    if a > b {"Valid"} else {"Invalid"}
}

pub fn max_pair_difference_in_list (list: &[i32]) -> u32 {
    let min = list.iter().min().unwrap_or(&0);
    let max = list.iter().max().unwrap_or(&0);
    (*max - *min) as u32 
}

pub fn sum_of_first_n_positive_ints (num: u32) -> u64 {
    let n = num as u64;
    (n * (n + 1)) / 2
}

pub fn positive_even_negative_odd (num: i32) -> &'static str {
    if num % 2 == 0 && num.is_positive() { return "+even" }
    if num % 2 == -1 { return "-odd" }

    "+number-"
}

pub fn count_negatives_in_list (list: &[i32]) -> u32 {
    list.iter().filter(|&n| n.is_negative()).count() as u32
}

pub fn sum_of_even_in_list (list: &[i32]) -> i32 {
    list.iter().filter(|&n| n % 2 == 0).sum::<i32>()
}

pub fn multiplication_table_to_ten (num: i32) -> String {
    let table: [i32; 10] = std::array::from_fn(|i| (i as i32 + 1) * num);

    format!("{table:?}")
}

pub fn terminals_of_list (list: &[i32]) -> String {
    format!("[{}, {}]", list[0], list[list.len() - 1])
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

    #[test]
    fn equal_digits() {
        let posi1 = last_digit_c_equals_of_ab(25, 21, 255);
        let posi2 = last_digit_c_equals_of_ab(55, 226, 5090);
        let nega1 = last_digit_c_equals_of_ab(12, 215, 2142);
        assert_eq!(posi1, true);
        assert_eq!(posi2, true);
        assert_eq!(nega1, false);
    }

    #[test]
    fn count_quad_roots() {
        let zero = count_distinct_quadratic_roots(1, 0, 1);
        let one = count_distinct_quadratic_roots(1, 0, 0);
        let two = count_distinct_quadratic_roots(1, 0, -1);

        assert_eq! (0u8, zero);
        assert_eq! (1u8, one);
        assert_eq! (2u8, two);
    }

    #[test]
    fn test_pair_diff_in_list () {
        let three = max_pair_difference_in_list(&[2, 1, 0, -1]);
        let thirteen = max_pair_difference_in_list(&[2, 4, 7, 2, -2, -6, 7]);

        assert_eq! (3, three);
        assert_eq! (13, thirteen);
    }

}