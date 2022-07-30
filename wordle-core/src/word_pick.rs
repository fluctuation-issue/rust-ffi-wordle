//! Select words to use as input of wordle game.

/// Choose a word.
pub trait WordPicker {
    /// Choose a single word to play.
    fn pick_word(&mut self) -> String;
}

/// Generate implementation of std::iter::IntoIterator for the specified type.
///
/// The result will be of type [WordPickerIter].
macro_rules! impl_into_word_picker_iter {
    ($struct_type: ident) => {
        impl std::iter::IntoIterator for $struct_type {
            type Item = String;
            type IntoIter = WordPickerIter<Self>;

            fn into_iter(self) -> Self::IntoIter {
                WordPickerIter {
                    picker: self
                }
            }
        }
    }
}

/// An infinite iterator that produces words to use as wordle game target.
pub struct WordPickerIter<P> {
   picker: P, 
}

impl<T: WordPicker> std::iter::Iterator for WordPickerIter<T> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.picker.pick_word())
    }
}

/// Choose a word in a list.
///
/// Goes from first word to last.
/// Wrap to the first word once the end has been reached.
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ListWordPicker {
    words: Vec<String>,
    current_word_index: usize,
}

impl_into_word_picker_iter!(ListWordPicker);

/// Could not convert an iterator over strings to a [ListWordPicker].
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum ListWordPickerFromIteratorError {
    /// The list constituted from the iterator did not contain any valid word.
    NoWords
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct ListWordPickerFromIterator(Result<ListWordPicker, ListWordPickerFromIteratorError>);

impl<S: AsRef<str>> std::iter::FromIterator<S> for ListWordPickerFromIterator {
    fn from_iter<I: std::iter::IntoIterator<Item=S>>(iter: I) -> Self {
        let words: Vec<String> = iter.into_iter().map(|word| word.as_ref().to_string()).collect();
        let result = if words.is_empty() {
            Err(ListWordPickerFromIteratorError::NoWords)
        } else {
            Ok(ListWordPicker {
                words,
                current_word_index: 0,
            })
        };
        Self(result)
    }
}

impl WordPicker for ListWordPicker {
    fn pick_word(&mut self) -> String {
        let word = self.words[self.current_word_index].clone();
        self.current_word_index = (self.current_word_index + 1) % self.words.len();
        word
    }
}

/// Pick a random word from a list.
pub struct RandomWordPicker {
    words: Vec<String>,
    rng: rand::rngs::ThreadRng
}

/// Error occurred while initializing [RandomWordPicker] with a file path.
#[cfg_attr(test, derive(Debug))]
pub enum RandomWordPickerError {
    /// Input/output error while reading the file.
    Io(std::io::Error),
    /// The file did not contain any word.
    NoWords
}

impl RandomWordPicker {
    /// Try to load words from the specified file.
    ///
    /// Will fail if the file can not be opened.
    /// Words must be an a separate line.
    /// Empty lines are skipped.
    pub fn from_path<Path: AsRef<std::path::Path>>(words_file_path: Path) -> Result<Self, RandomWordPickerError> {
        let words_file = std::fs::File::open(words_file_path.as_ref()).map_err(RandomWordPickerError::Io)?;
        let reader = std::io::BufReader::new(words_file);
        Self::from_reader(reader)
    }

    /// Load words from a reader.
    ///
    /// Each line should contain a single word.
    /// Empty lines are skipped.
    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, RandomWordPickerError> {
        use std::io::BufRead;
        let reader = std::io::BufReader::new(reader);
        let words = reader.lines().filter_map(|line| match line {
            Err(error) => Some(Err(error)),
            Ok(line) => if line.is_empty() {
                None
            } else {
                Some(Ok(line))
            }
        }).collect::<std::io::Result<Vec<String>>>().map_err(RandomWordPickerError::Io)?;
        if words.is_empty() {
            Err(RandomWordPickerError::NoWords)
        } else {
            Ok(Self {
                words,
                rng: rand::thread_rng()
            })
        }
    }
}

impl WordPicker for RandomWordPicker {
    /// Pick a random word from the list created using the input file.
    fn pick_word(&mut self) -> String {
        use rand::seq::SliceRandom;
        self.words[..].choose(&mut self.rng).expect("words must not be empty").clone()
    }
}

impl_into_word_picker_iter!(RandomWordPicker);

/// A word picker.
#[repr(C)]
pub struct WordPickerT {
    /// Generic pointer to the word picker.
    this: *mut std::ffi::c_void,
    /// Pointer to a function picking the word.
    pick_word: fn(*mut std::ffi::c_void) -> *mut std::os::raw::c_char,
}

/// Create a word picker of the correct type to pick the next word.
///
/// The result word must be freed with [crate::game::rust_str_free].
fn pick_word_generic<T: WordPicker>(picker: *mut std::ffi::c_void) -> *mut std::os::raw::c_char {
    let picker = {
        assert!(!picker.is_null());
        unsafe { &mut*(picker as *mut T) }
    };
    let word = std::ffi::CString::new(picker.pick_word()).unwrap();
    word.into_raw()
}

/// Create a new word picker that chooses from a list.
///
/// # Safety
/// `words` must be a `NULL`-terminated array of strings.
#[no_mangle]
pub unsafe extern "C" fn wc_word_picker_new_from_list(mut words: *const *const std::os::raw::c_char) -> *mut WordPickerT {
    if words.is_null() {
        return std::ptr::null_mut();
    }

    let mut list = Vec::new();
    while !(*words).is_null() {
        let current_string = std::ffi::CStr::from_ptr(*words).to_string_lossy();
        list.push(current_string);
        words = words.add(1);
    }

    let inner_picker = match ListWordPickerFromIterator::from_iter(list).0 {
        Err(_) => return std::ptr::null_mut(),
        Ok(picker) => picker,
    };
    let inner_picker = Box::into_raw(Box::new(inner_picker)) as *mut std::ffi::c_void;
    let picker = WordPickerT {
        this: inner_picker,
        pick_word: pick_word_generic::<ListWordPicker>,
    };
    Box::into_raw(Box::new(picker))
}

/// C wrapper to create a new word picker, using random lines from a file.
///
/// # Safety
///
/// `file_path` must be a valid pointer to a `NULL`-terminated string.
#[no_mangle]
pub unsafe extern "C" fn wc_word_picker_new_random_line_file(file_path: *const std::os::raw::c_char) -> *mut WordPickerT {
    let file_path = std::ffi::CStr::from_ptr(file_path).to_string_lossy().to_string();
    let inner_picker = match RandomWordPicker::from_path(&file_path) {
        Err(_) => return std::ptr::null_mut(),
        Ok(picker) => Box::into_raw(Box::new(picker)) as *mut std::ffi::c_void,
    };
    let picker = WordPickerT {
        this: inner_picker,
        pick_word: pick_word_generic::<RandomWordPicker>,
    };
    Box::into_raw(Box::new(picker))
}

/// C wrapper to pick a word, from a picker.
///
/// # Safety
/// `picker` must be a valid picker.
///
/// The result word must by freed with [crate::game::rust_str_free].
#[no_mangle]
pub unsafe fn wc_word_picker_pick_word(picker: *mut WordPickerT) -> *mut std::os::raw::c_char {
    ((*picker).pick_word)((*picker).this)
}

/// C wrapper to free a word picker.
///
/// # Safety
/// `picker` is readonly.
#[no_mangle]
pub unsafe extern "C" fn wc_word_picker_free(picker: *mut WordPickerT) {
    if picker.is_null() {
        return
    }
    let picker = Box::from_raw(picker);
    let _ = Box::from_raw(picker.this);
}

#[cfg(test)]
mod tests {
    use super::{WordPicker, ListWordPickerFromIterator, ListWordPickerFromIteratorError, RandomWordPicker};

    #[test]
    fn list_word_picker_from_empty_list() {
        assert_eq!(
            ListWordPickerFromIterator::from_iter::<[&str; 0]>([]),
            ListWordPickerFromIterator(Err(ListWordPickerFromIteratorError::NoWords))
        );
    }

    #[test]
    fn list_word_picker_from_list() {
        let list = ListWordPickerFromIterator::from_iter(["this", "is", "a", "test"]).0.unwrap();
        assert_eq!(list.words, vec![String::from("this"), String::from("is"), String::from("a"), String::from("test")]);
        assert_eq!(list.current_word_index, 0);
    }

    #[test]
    fn list_word_implements_word_picker() {
        let mut list: Box<dyn WordPicker> = Box::new(ListWordPickerFromIterator::from_iter(["this", "is", "a", "test"]).0.unwrap());
        assert_eq!(list.pick_word(), String::from("this"));
        assert_eq!(list.pick_word(), String::from("is"));
        assert_eq!(list.pick_word(), String::from("a"));
        assert_eq!(list.pick_word(), String::from("test"));
        assert_eq!(list.pick_word(), String::from("this"));
        assert_eq!(list.pick_word(), String::from("is"));
        assert_eq!(list.pick_word(), String::from("a"));
        assert_eq!(list.pick_word(), String::from("test"));
    }

    #[test]
    fn list_word_implements_iterator() {
        let mut list = ListWordPickerFromIterator::from_iter("this is a test".split_whitespace()).0.unwrap().into_iter();
        assert_eq!(list.next(), Some(String::from("this")));
        assert_eq!(list.next(), Some(String::from("is")));
        assert_eq!(list.next(), Some(String::from("a")));
        assert_eq!(list.next(), Some(String::from("test")));
        assert_eq!(list.next(), Some(String::from("this")));
    }

    #[test]
    fn random_word_picker_from_cursor() {
        let cursor = std::io::Cursor::new("temp\ntest\ndone\n\nprevious line is empty");
        let picker = RandomWordPicker::from_reader(cursor).expect("no io error from cursor");
        assert_eq!(picker.words, vec![String::from("temp"), String::from("test"), String::from("done"), String::from("previous line is empty")]);
    }
}
