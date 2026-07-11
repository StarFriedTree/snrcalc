pub fn search_int_or_potential_index_in_sorted (item: i32, list: &[i32]) -> i32 {
    match list.binary_search(&item) {
        Ok(_) => item,
        Err(index) => index as i32,
    }
}

pub fn digits_in_reverse (mut num: u32) -> String {
    let mut digits = Vec::<u8>::with_capacity(9);
    loop {
        digits.push((num % 10) as u8);
        num /= 10;
        if num == 0 { break; }
    }
    format!{"{digits:?}"}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_search_or_index () {
        let one = search_int_or_potential_index_in_sorted(1, &[-1, 0, 1, 2]);
        let two = search_int_or_potential_index_in_sorted(2, &[0, 1, 3, 4]);

        assert_eq!(one, 1);
        assert_eq!(two, 2);
    }
}