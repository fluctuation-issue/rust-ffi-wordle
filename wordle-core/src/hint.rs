//! Hints that sould be displayed to the user, after a guess.

use std::collections::HashMap;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
/// Hint for a guess.
pub enum LetterHint {
    /// The letter in the guess word is the same as in the target word.
    Correct = 0,
    /// The letter appears in the target word but is not placed correctly in the guess word.
    PlacementIncorrect,
    /// The letter does not appear in the target word.
    Incorrect,
}

/// Hints for a guessed word.
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct GuessHint<'a> {
    guessed: &'a str,
    word_to_guess: &'a str,
}

impl<'a> GuessHint<'a> {
    /// Initialize a new guess hint.
    ///
    /// The guessed word must be in uppercase.
    ///
    /// # Panics
    ///
    /// If the guessed word is not in uppercase.
    pub(crate) fn new(guessed: &'a str, word_to_guess: &'a str) -> Self {
        assert_eq!(guessed, guessed.to_uppercase().as_str());
        Self::assert_guess_and_target_words_are_correct(guessed, word_to_guess);
        Self {
            guessed,
            word_to_guess,
        }
    }

    /// Make sure that guessed word and word to guess are correct.
    ///
    /// Neither word can be empty.
    /// They must have the same length.
    fn assert_guess_and_target_words_are_correct(guessed: &'a str, word_to_guess: &'a str) {
        if guessed.is_empty() {
            panic!("Guessed word must not be empty.");
        } else if word_to_guess.is_empty() {
            panic!("Word to guess must not be empty.");
        } else if guessed.len() != word_to_guess.len() {
            panic!("Guess and target words must have the same length.");
        }
    }

    /// Returns the guessed word.
    ///
    /// It will be in uppercase.
    pub fn guessed(&self) -> &'a str {
        self.guessed
    }

    /// Get the letter hints for the guessed word.
    pub fn letter_hints(&self) -> Vec<LetterHint> {
        let mut result = vec![LetterHint::Incorrect; self.guessed.len()];
        let are_correct = self
            .guessed
            .chars()
            .zip(self.word_to_guess.chars())
            .map(|(guess, target)| guess == target);
        for (letter_hint, is_correct) in result.iter_mut().zip(are_correct) {
            if is_correct {
                *letter_hint = LetterHint::Correct;
            }
        }

        let letters_to_guess_that_are_incorrect = self
            .word_to_guess
            .chars()
            .zip(result.iter())
            .filter_map(|(character, hint)| {
                if !matches!(hint, LetterHint::Correct) {
                    Some(character)
                } else {
                    None
                }
            });
        let target_letter_occurrences = get_letter_occurrences(letters_to_guess_that_are_incorrect);

        let mut current_occurrences = HashMap::new();
        for (i, guess_character) in self.guessed.chars().enumerate() {
            if matches!(result[i], LetterHint::Correct) {
                continue;
            }

            match current_occurrences.get_mut(&guess_character) {
                Some(count) => *count += 1,
                None => {
                    current_occurrences.insert(guess_character, 1);
                }
            }

            result[i] = if current_occurrences.get(&guess_character).unwrap()
                <= target_letter_occurrences
                    .get(&guess_character)
                    .unwrap_or(&0)
            {
                LetterHint::PlacementIncorrect
            } else {
                LetterHint::Incorrect
            };
        }
        result
    }

    /// Associate each letter hint with its matching guessed letter.
    ///
    /// See [GuessHint::letter_hints()].
    pub fn guessed_letters_and_hints(&self) -> Vec<(char, LetterHint)> {
        self.guessed.chars().zip(self.letter_hints()).collect()
    }
}

/// Count the number of times a letter appears in a string.
fn get_letter_occurrences<L: std::iter::Iterator<Item = char>>(letters: L) -> HashMap<char, usize> {
    letters.fold(HashMap::new(), |mut acc, character| {
        match acc.get_mut(&character) {
            Some(count) => *count += 1,
            None => {
                acc.insert(character, 1);
            }
        }
        acc
    })
}

/// C wrapper to represent [GuessHint].
#[repr(C)]
pub struct GuessHintT {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// C wrapper to represent an array of [LetterHint].
#[repr(C)]
pub struct LetterHints {
    hints: *mut LetterHint,
    num_hints: u32,
}

/// C wrapper to represent an array of guessed letters and hints.
///
/// See [GuessHint::guessed_letters_and_hints()], [wc_guess_hint_get_guessed_letters_and_hints].
#[repr(C)]
pub struct GuessedLetterAndHint {
    letter: char,
    hint: LetterHint,
}

/// C wrapper to represent a linked list of [GuessedLetterAndHint].
#[repr(C)]
pub struct GuessedLettersAndHints {
    letters_and_hints: *mut GuessedLetterAndHint,
    num_letters_and_hints: u32,
}

/// C wrapper to get the guessed word.
///
/// The result must be freed by calling [super::game::rust_str_free()].
#[no_mangle]
pub extern "C" fn wc_guess_hint_get_guessed(
    guess_hint: *const GuessHintT,
) -> *mut std::os::raw::c_char {
    let guess_hint = {
        assert!(!guess_hint.is_null());
        unsafe { &*(guess_hint as *const GuessHint) }
    };
    let word_to_guess = std::ffi::CString::new(guess_hint.guessed).unwrap();
    word_to_guess.into_raw()
}

/// C wrapper to get the letter hints.
///
/// Must be freed by [wc_guess_hint_free_letter_hints()].
///
/// See [GuessHint::letter_hints()].
#[no_mangle]
pub extern "C" fn wc_guess_hint_get_letter_hints(
    guess_hint: *const GuessHintT,
) -> *mut LetterHints {
    let guess_hint = {
        assert!(!guess_hint.is_null());
        unsafe { &*(guess_hint as *const GuessHint) }
    };
    let mut hints_slice: Box<[LetterHint]> = guess_hint.letter_hints().into_boxed_slice();
    let hints = hints_slice.as_mut_ptr();
    let num_hints = hints_slice.len() as u32;
    std::mem::forget(hints_slice);

    let letter_hints = LetterHints { hints, num_hints };
    Box::into_raw(Box::new(letter_hints)) as *mut LetterHints
}

/// # Safety
/// `letter_hints` is read only.
/// It must be allocated by [wc_guess_hint_get_letter_hints].
#[no_mangle]
pub unsafe extern "C" fn wc_guess_hint_free_letter_hints(letter_hints: *mut LetterHints) {
    let letter_hints = Box::from_raw(letter_hints);
    let _ = std::slice::from_raw_parts_mut(letter_hints.hints, letter_hints.num_hints as usize);
}

/// C wrapper to get the guessed letters and hints.
///
/// Must be freed by [wc_guess_hint_free_guessed_letters_and_hints()].
#[no_mangle]
pub extern "C" fn wc_guess_hint_get_guessed_letters_and_hints(
    guess_hint: *const GuessHintT,
) -> *mut GuessedLettersAndHints {
    let guess_hint = {
        assert!(!guess_hint.is_null());
        unsafe { &*(guess_hint as *const GuessHint) }
    };
    let mut letters_and_hints_slice: Box<[GuessedLetterAndHint]> = guess_hint
        .guessed_letters_and_hints()
        .into_iter()
        .map(|(letter, hint)| GuessedLetterAndHint { letter, hint })
        .collect::<Vec<GuessedLetterAndHint>>()
        .into_boxed_slice();
    let letters_and_hints = letters_and_hints_slice.as_mut_ptr();
    let num_letters_and_hints = letters_and_hints_slice.len() as u32;
    std::mem::forget(letters_and_hints_slice);

    let guessed_letters_and_hints = GuessedLettersAndHints {
        letters_and_hints,
        num_letters_and_hints,
    };
    Box::into_raw(Box::new(guessed_letters_and_hints)) as *mut GuessedLettersAndHints
}

/// # Safety
/// `guessed_letters_and_hints` is read only.
/// It must be allocated by [wc_guess_hint_get_guessed_letters_and_hints].
#[no_mangle]
pub unsafe extern "C" fn wc_guess_hint_free_guessed_letters_and_hints(
    guessed_letters_and_hints: *mut GuessedLettersAndHints,
) {
    let guessed_letters_and_hints = Box::from_raw(guessed_letters_and_hints);
    let _ = std::slice::from_raw_parts_mut(
        guessed_letters_and_hints.letters_and_hints,
        guessed_letters_and_hints.num_letters_and_hints as usize,
    );
}

#[cfg(test)]
mod tests {
    use super::{get_letter_occurrences, GuessHint, LetterHint};

    #[test]
    #[should_panic]
    fn guess_hint_new_empty_target_panics() {
        GuessHint::new("hello", "");
    }

    #[test]
    #[should_panic]
    fn guess_hint_new_empty_guess_panics() {
        GuessHint::new("", "hi");
    }

    #[test]
    #[should_panic]
    fn guess_hint_different_length_panics() {
        GuessHint::new("hello", "hi");
    }

    #[test]
    fn guess_hint_letters_hint() {
        let hint = GuessHint::new("AXXXX", "AAAAA");
        let letter_hints = hint.letter_hints();
        assert_eq!(
            letter_hints,
            vec![
                LetterHint::Correct,
                LetterHint::Incorrect,
                LetterHint::Incorrect,
                LetterHint::Incorrect,
                LetterHint::Incorrect
            ]
        );

        let hint = GuessHint::new("AXBCB", "AACBB");
        let letter_hints = hint.letter_hints();
        assert_eq!(
            letter_hints,
            vec![
                LetterHint::Correct,
                LetterHint::Incorrect,
                LetterHint::PlacementIncorrect,
                LetterHint::PlacementIncorrect,
                LetterHint::Correct
            ]
        );

        let hint = GuessHint::new("AAAAB", "AACBB");
        let letter_hints = hint.letter_hints();
        assert_eq!(
            letter_hints,
            vec![
                LetterHint::Correct,
                LetterHint::Correct,
                LetterHint::Incorrect,
                LetterHint::Incorrect,
                LetterHint::Correct
            ]
        );

        let hint = GuessHint::new("ABBBB", "AACBB");
        let letter_hints = hint.letter_hints();
        assert_eq!(
            letter_hints,
            vec![
                LetterHint::Correct,
                LetterHint::Incorrect,
                LetterHint::Incorrect,
                LetterHint::Correct,
                LetterHint::Correct
            ]
        );

        let hint = GuessHint::new("ABBAB", "ABBBA");
        let letter_hints = hint.letter_hints();
        assert_eq!(
            letter_hints,
            vec![
                LetterHint::Correct,
                LetterHint::Correct,
                LetterHint::Correct,
                LetterHint::PlacementIncorrect,
                LetterHint::PlacementIncorrect
            ]
        );
    }

    #[test]
    fn test_get_letter_occurrences() {
        let occurrences = get_letter_occurrences("abcda".chars());
        assert_eq!(occurrences.get(&'a'), Some(&2));
        assert_eq!(occurrences.get(&'b'), Some(&1));
        assert_eq!(occurrences.get(&'c'), Some(&1));
        assert_eq!(occurrences.get(&'d'), Some(&1));
        assert_eq!(occurrences.get(&'e'), None);
    }
}
