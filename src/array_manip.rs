pub fn search_int_or_potential_index_in_sorted (item: i32, list: &[i32]) -> i32 {
    match list.binary_search(&item) {
        Ok(_) => item,
        Err(index) => index as i32,
    }
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