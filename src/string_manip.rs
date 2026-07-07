pub fn syllable_count_in_hyphenated_word (word: &str) -> i32 {
    word.split('-').filter(|&syllable| !syllable.is_empty()).count() as i32
}

pub fn reverse_string (text: &str) -> String {
    text.chars().rev().collect()
}

