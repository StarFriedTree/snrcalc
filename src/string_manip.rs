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

pub fn is_alpha_only (text: &str) -> bool {
    text.chars().all(|c| c.is_alphabetic())
}

pub fn replace_vowels (text: &str, replacement: &char) -> String {
    text.chars().map(|c| if "aeiouAEIOU".contains(c) {*replacement} else {c}).collect()
}

pub fn shift_cipher_one (text: &str) -> String {
    // let mut result = String::with_capacity(text.len());
    // for c in text.chars() {
    //     if c.is_ascii_alphabetic() && c != 'z' && c != 'Z' {
    //         result.push((c as u8 + 1) as char);
    //     } 
    //     else if c == 'z' || c == 'Z' {
    //         result.push((c as u8 - 25) as char);
    //     } 
    //     else {
    //         result.push(c);
    //     }
    // }
    // result
    // old solution ^ idk which one's better tho

    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() {b'A'} else {b'a'};

                (((c as u8 - base + 1) % 26) + base) as char
            } else {c}
        })
        .collect()
    
}

pub fn check_palindrome (text: &str) -> bool {
    let cleaned = text.chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_lowercase());

    cleaned.clone().eq(cleaned.rev())
}

pub fn filter_4letter_words_from_list (list: &[&str]) -> String {
    format!( "{:?}",
        list.iter().filter(|word| word.len() == 4).collect::<Vec::<_>>()
    )
}

pub fn same_amount_of_x_o (text: &str) -> bool {
    text.bytes()
        .fold(0, |balance: i32, c| match c {
            b'x' | b'X' => balance + 1,
            b'o' | b'O' => balance - 1,
            _ => balance,
        }) == 0
}

pub fn longest_word_in_sentence (text: &str) -> String {
    text.trim()
        .split_ascii_whitespace()
        .max_by_key(|word| 
                word.chars().filter(|c| !c.is_ascii_punctuation()).count()
            )
        .unwrap_or("")
        .to_string()
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