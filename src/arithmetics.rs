use std::cmp::Ordering;
use regex::Regex;

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

pub fn count_three_equal (a: i32, b: i32, c: i32) -> u8 {
    if a == b && b == c {3}
    else if a != b && b != c && c != a {0}
    else {2}
}

pub fn multiplication_table_to_n (num: i32, len: u16) -> String{
    let mut table: Vec<i32> = Vec::with_capacity(len as usize);
    for i in 1..=len as i32 {
        table.push(i * num);
    }
    format!("{table:?}")
}

/// A milk carton can hold 3.78 liters of milk. Each morning, a dairy farm ships cartons
/// of milk to a local grocery store. The cost of producing one liter of milk is $0.038, and
/// the profit of each carton of milk is $0.27. Write a program that does the following:
/// 1. Prompts the user to enter the total amount of milk produced in the morning.
/// 2. Outputs the number of milk cartons needed to hold milk. (Round your answer to
/// the nearest integer)
/// 3. Outputs the cost of producing milk.
/// 4. Outputs the profit for producing milk.
pub fn milk_cartons_cost_profit (milk_amount: f64) -> String {
    let cartons = (milk_amount / 3.78).round();
    let cost = milk_amount * 0.038;
    let profit = cartons * 0.27;
    format!("{cartons} cartons; ${cost} in costs; ${profit} in profit")
}

pub fn sum_binary_to_binary (a: &[u8], b: &[u8]) -> String {
    let (lrg, sml) = if a.len() < b.len() {(b, a)} else {(a, b)};

    let zipped = lrg.iter().rev().zip(sml.iter().rev().chain(std::iter::repeat(&0)));

    let mut result = Vec::<u8>::with_capacity(lrg.len() + 1);
    let mut carry = 0u8;

    for (&l, &s) in zipped {
        let (sum, overflow1) = l.overflowing_add(s);
        let (sum, overflow2) = sum.overflowing_add(carry);
        result.push(sum);
        carry = (overflow1 || overflow2) as u8;
    }
    if carry != 0 {
        result.push(carry);
    }

    result
        .iter()
        .rev()
        .map(|byte| format!("{byte:08b}"))
        .collect::<Vec<String>>()
        .join("_")
}

static ROMAN_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^m{0,3}(cm|cd|d?c{0,3})(xc|xl|l?x{0,3})(ix|iv|v?i{0,3})$").unwrap()
});

pub fn is_valid_roman_numeral (num: &str) -> bool {
    // if !num
    //     .bytes ()
    //     .all (|b| 
    //         matches!(b.to_ascii_lowercase(), b'i' | b'v' | b'x' | b'l' | b'c' | b'd' | b'm')
    //     ) {
    //     return false;
    // }

    // let lower = num.to_ascii_lowercase();
    // let forbidden_patterns = [
    //     "iiii", "xxxx", "cccc", "mmmm",
    //     "vv", "ll", "dd",
    //     "iiv", "iix", "il", "ic", "id", "im",
    //     "ivi", "ixi", "xlx", "xcx",
    //     "vx", "vl", "vc", "vd", "vm",
    //     "xxl", "xxc", "xd", "xm",
    //     "lc", "ld", "lm",
    //     "ccd", "ccm", "cdc", "cmc",
    //     "dm",
    // ];
    // if forbidden_patterns.iter().any(|p| lower.contains(p)) {
    //     return false;
    // }

    // true
    ROMAN_RE.is_match(&num.to_ascii_lowercase())
}

pub fn roman_numeral_to_int (num: &str) -> u16 {
    if !is_valid_roman_numeral (num) { return 0; }

    let lower = num.to_ascii_lowercase();

    // let mut iter = lower.chars();

    let mut result = 0i16;
    let mut iter = lower.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            'm' => result += 1000,
            'c' => {
                if iter.peek() == Some(&'m') || iter.peek() == Some(&'d') {
                    result -= 100;
                } else { result += 100; }
            },
            'd' => result += 500,
            'x' => {
                if iter.peek() == Some(&'c') || iter.peek() == Some(&'l') {
                    result -= 10;
                } else { result += 10; }
            },
            'l' => result += 50,
            'i' => {
                if iter.peek() == Some(&'x') || iter.peek() == Some(&'v') {
                    result -= 1;
                } else { result -= 1; }
            },
            'v' => result += 5,
            _ => panic!("did not find valid roman numeral while converting to int"),
        }
    }
    result as u16
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