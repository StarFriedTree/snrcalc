pub fn syllable_count_in_hyphenated_word (word: &str) -> i32 {
    word.split('-').filter(|&syllable| !syllable.is_empty()).count() as i32
}

pub fn reverse_string (text: &str) -> String {
    text.chars().rev().collect()
}

pub fn burp_nr (r_count: usize) -> String {
    let r = "r".repeat(r_count);
    format!("Bu{r}p")
}

pub fn solid_clump_of_hashes (clump: &str) -> bool {
    for &c in clump.trim().as_bytes() {
        if c != b'#' { return false; }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_clump() {
        let positive = solid_clump_of_hashes("  ########");
        let negative = solid_clump_of_hashes("#### ####");
        assert_eq!(positive, true);
        assert_eq!(negative, false);
    }
}