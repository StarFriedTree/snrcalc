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

pub fn count_words_in_sentence (sentence: &str) -> i32 {
    sentence.trim().split(" ").count() as i32
}

pub fn vowel_counter (text: &str) -> u32 {
    text.chars().filter(|&b| "aeiouAEIOU".contains(b)).count() as u32
}

pub fn repeat_each_char (text: &str) -> String {
    text.chars().flat_map(|c| [c, c]).collect()
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

    #[test]
    fn test_vowel_counter() {
        let one = vowel_counter("bob");
        let three = vowel_counter ("Eleven");
        assert_eq! (1, one);
        assert_eq! (3, three);
    }

    #[test]
    fn test_char_repeater () {
        let hello = repeat_each_char ("hello");
        let dragon = repeat_each_char("Dragon");

        assert_eq! ("hheelllloo", hello);
        assert_eq! ("DDrraaggoonn", dragon);
    }

}