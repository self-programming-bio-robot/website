use std::collections::HashSet;

use lazy_static::lazy_static;

lazy_static! {
    static ref SPECIAL: HashSet<char> =HashSet::from(
        ['.', ',', '!', '?', ':', ';', ']', ')', '}', '>', '"', '\'', '>', '%']
    );
}

pub fn split_line_by_limit(text: &str, limit: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = text.char_indices();
    let mut text_reminder = text;

    while let Some((index, _)) = chars.nth(limit) {
        let current_part = &text_reminder[..index];
        let until_splitter = trim_string_until_splitter(current_part);

        if let Some(until_splitter) = until_splitter {
            result.push(until_splitter.to_string());
            text_reminder = &text_reminder[until_splitter.len()..];
            chars = text_reminder.char_indices();
        } else {
            result.push(current_part.to_string());
            text_reminder = &text_reminder[current_part.len()..];
            chars = text_reminder.char_indices();
        }
        while let Some((index, c)) = chars.next() {
            if c != ' ' {
                text_reminder = &text_reminder[index..];
                chars = text_reminder.char_indices();
                break;
            }
        }
    }
    if !text_reminder.is_empty() {
        result.push(text_reminder.to_string());
    }
    result
}

pub fn stretch_text(text: &str, expected_length: usize) -> String {
    if (text.chars().count() as f32 / expected_length as f32) < 0.8 {
        return text.to_string();
    }
    
    let words: Vec<&str> = split_text_into_words(text);
    let current_length: usize = words.iter().map(|word| word.chars().count()).sum();
    let num_gaps = words.len() - 1;
    let total_spaces_needed = expected_length - current_length;

    if total_spaces_needed <= 0 || num_gaps == 0 {
        return text.to_string();
    }

    let spaces_per_gap = total_spaces_needed / num_gaps;
    let mut remaining_spaces = total_spaces_needed % num_gaps;

    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        result.push_str(word);
        if i < num_gaps {
            for _ in 0..spaces_per_gap {
                result.push(' ');
            }
            if remaining_spaces > 0 {
                result.push(' ');
                remaining_spaces -= 1;
            }
        } else {
            while result.len() < expected_length {
                result.push(' ');
            }
        }
    }

    result
}

fn trim_string_until_splitter(text: &str) -> Option<&str> {
    let mut chars = text.char_indices();
    while let Some((index, c)) = chars.next_back() {
        if c == ' ' {
            return Some(&text[..index]);
        }
        if SPECIAL.contains(&c) {
            return Some(&text[..index + 1]);
        }
    }
    return None;
}

fn split_text_into_words(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return vec![text];
    }
    
    let divided_marks: HashSet<char> = HashSet::from([',', '.', '!', '?', ':', ';', ']', ')', '}', '>']);
    let break_points = text.char_indices()
        .collect::<Vec<(usize, char)>>()
        .windows(2)
        .filter(|window| {
            let (_, c) = window[0];
            let (_, next_c) = window[1];
            let mark_c = divided_marks.contains(&c);
            let mark_next_c = divided_marks.contains(&next_c);
            
            (c == ' ' || mark_c) && next_c == ' ' 
                || c == ' ' 
                || mark_c && !mark_next_c
        })
        .map(|window| window[1].0)
        .collect::<Vec<usize>>();
    
    let mut result = Vec::new();
    let mut start = 0;

    for index in break_points {
        if index <= text.len() {
            let substring = &text[start..index];
            if substring != " " {
                result.push(substring);
            }
            start = index;
        }
    }

    if start < text.len() {
        let substring = &text[start..];
        if substring != " " {
            result.push(substring);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_text_by_limit_splits_correctly() {
        let text = "Hello, Число!";
        let limit = 6;
        let expected = vec!["Hello,", "Число!"];
        assert_eq!(split_line_by_limit(&text, limit), expected);
    }

    #[test]
    fn split_text_by_limit_handles_empty_string() {
        let text = "";
        let limit = 5;
        let expected: Vec<String> = Vec::new();
        assert_eq!(split_line_by_limit(&text, limit), expected);
    }

    #[test]
    fn split_text_by_limit_handles_limit_greater_than_text_length() {
        let text = "Hello";
        let limit = 10;
        let expected = vec!["Hello"];
        assert_eq!(split_line_by_limit(&text, limit), expected);
    }

    #[test]
    fn split_text_by_limit_handles_limit_equal_to_text_length() {
        let text = "Hello";
        let limit = 5;
        let expected = vec!["Hello"];
        assert_eq!(split_line_by_limit(&text, limit), expected);
    }

    #[test]
    fn split_text_by_limit_handles_limit_less_than_word_length() {
        let text = "Hello, Число!";
        let limit = 3;
        let expected = vec!["Hel", "lo,", "Чис", "ло!"];
        assert_eq!(split_line_by_limit(&text, limit), expected);
    }

    #[test]
    fn split_text_by_limit_trimming_extra_spaces() {
        let text = "Hello,     Число!";
        let limit = 6;
        let expected = vec!["Hello,", "Число!"];
        assert_eq!(split_line_by_limit(&text, limit), expected);
    }

    #[test]
    fn find_splitter_finds_space() {
        let text = "Hello, World";
        let expected = Some("Hello,");
        assert_eq!(trim_string_until_splitter(&text), expected);
    }

    #[test]
    fn find_splitter_finds_space_at_last_position() {
        let text = "Hello, World ";
        let expected = Some("Hello, World");
        assert_eq!(trim_string_until_splitter(&text), expected);
    }

    #[test]
    fn find_splitter_special_symbol() {
        let text = "Hello,World";
        let expected = Some("Hello,");
        assert_eq!(trim_string_until_splitter(&text), expected);
    }

    #[test]
    fn not_found_splitter() {
        let text = "HelloWorld";
        let expected = None;
        assert_eq!(trim_string_until_splitter(&text), expected);
    }

    #[test]
    fn find_splitter_with_cyrillic_characters() {
        let text = "Привет, Мир";
        let expected = Some("Привет,");
        assert_eq!(trim_string_until_splitter(&text), expected);
    }
}

#[cfg(test)]
mod stretch_text_tests {
    use crate::string_utils::stretch_text;

    #[test]
    fn stretch_text_if_witdh_approach_to_max_length() {
        let text = "Привет,Мир";
        let expected = "Привет,  Мир";
        assert_eq!(stretch_text(&text, 12), expected);
    }

    #[test]
    fn dont_stretch_text_if_witdh_too_little() {
        let text = "Привет, Мир";
        let expected = "Привет, Мир";
        assert_eq!(stretch_text(&text, 20), expected);
    }
}

#[cfg(test)]
mod split_text_into_words_tests {
    use crate::string_utils::split_text_into_words;

    #[test]
    fn split_text_into_words_splits_on_whitespace_and_divided_marks() {
        let text = "Hello, world!";
        let expected = vec!["Hello,", "world!"];
        assert_eq!(split_text_into_words(text), expected);
    }

    #[test]
    fn split_text_into_cyrillic_words_splits_on_whitespace_and_divided_marks() {
        let text = "Привет, мир!";
        let expected = vec!["Привет,", "мир!"];
        assert_eq!(split_text_into_words(text), expected);
    }


    #[test]
    fn split_text_into_words_handles_empty_string() {
        let text = "";
        let expected: Vec<&str> = vec![""];
        assert_eq!(split_text_into_words(text), expected);
    }

    #[test]
    fn split_text_into_words_handles_string_with_no_divided_marks() {
        let text = "Hello world";
        let expected = vec!["Hello ", "world"];
        assert_eq!(split_text_into_words(text), expected);
    }

    #[test]
    fn split_text_into_words_handles_string_with_only_divided_marks() {
        let text = ",.!?:;])}>";
        let expected: Vec<&str> = vec![",.!?:;])}>"];
        assert_eq!(split_text_into_words(text), expected);
    }

    #[test]
    fn split_text_into_words_handles_string_with_consecutive_divided_marks() {
        let text = "Hello,, world!!";
        let expected = vec!["Hello,,", "world!!"];
        assert_eq!(split_text_into_words(text), expected);
    }
}