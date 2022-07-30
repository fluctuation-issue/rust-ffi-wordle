//! Rules for a single game.

use super::hint::{GuessHint, GuessHintT};

/// Wordle game.
#[cfg_attr(test, derive(Eq,PartialEq,Debug))]
pub struct Game {
    word_to_guess: String,
    guesses: Vec<String>,
    attempts_count_limit: usize,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Eq,PartialEq))]
/// A new game could not be instantiated.
pub enum GameNewError {
    /// The target word had a null length.
    WordToGuessEmpty,
    /// The allowed attempts (before loosing the game) count was invalid.
    AttemptsCountLimitNull
}

impl Game {
    /// New game, whose goal is to guess the specified word.
    pub fn new(word_to_guess: &str) -> Result<Self, GameNewError> {
        if word_to_guess.is_empty() {
            Err(GameNewError::WordToGuessEmpty)
        } else {
            Ok(Self {
                word_to_guess: word_to_guess.to_uppercase(),
                guesses: vec![],
                attempts_count_limit: 6,
           })
        }
    }

    /// New game with custom attempts count limit.
    pub fn new_with_attempts_count_limit(word_to_guess: &str, attempts_count_limit: usize) -> Result<Self, GameNewError> {
        if attempts_count_limit < 1 {
            Err(GameNewError::AttemptsCountLimitNull)
        } else {
            let mut result = Self::new(word_to_guess)?;
            result.attempts_count_limit = attempts_count_limit;
            Ok(result)
        }
    }

    /// Retrieve the current game state.
    pub fn state(&self) -> GameState {
        if self.guesses.last() == Some(&self.word_to_guess) {
            GameState::Won {
                attempts: self.guesses.len(),
            }
        } else if self.guesses.len() >= self.attempts_count_limit {
            GameState::Lost
        } else {
            GameState::Pending {
                attempts_remaining: self.attempts_count_limit - self.guesses.len(),
            }
        }
    }

    /// Attempt to perform a guess.
    ///
    /// On success, return the new game state.
    /// Otherwise, return the error that occurred.
    pub fn guess(&mut self, guess: &str) -> Result<GameState, GameGuessError> {
        let guess = guess.to_uppercase();
        if guess.len() != self.word_to_guess.len() {
            Err(GameGuessError::LengthInvalid {
                given: guess.len(),
                expected: self.word_to_guess.len(),
            })
        } else if self.guesses.iter().any(|guessed| guessed == &guess) {
            Err(GameGuessError::AlreadyPlayed)
        } else {
            self.guesses.push(guess);
            Ok(self.state())
        }
    }

    /// Get hints for guessed words, from oldest to newest.
    pub fn guess_hints(&self) -> impl std::iter::Iterator<Item = GuessHint<'_>> + '_ {
        self.guesses
            .iter()
            .map(|guess| GuessHint::new(guess, &self.word_to_guess).unwrap())
    }

    /// Get hints for the newest guessed word, if any.
    pub fn current_guess_hint(&self) -> Option<GuessHint<'_>> {
        self.guesses
            .last()
            .map(|guess| GuessHint::new(guess, &self.word_to_guess).unwrap())
    }

    /// Reference to the word to guess to win the game.
    pub fn word_to_guess(&self) -> &str {
        &self.word_to_guess
    }
}

/// Wordle game state.
#[derive(Eq, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum GameState {
    /// The game is not done yet.
    Pending {
        /// Number of attempts left before loosing the game.
        attempts_remaining: usize,
    },
    /// The game is won: congratulations.
    Won {
        /// Number of guesses performed to win the game.
        attempts: usize,
    },
    /// The game was lost: all allowed guesses have failed.
    Lost,
}

/// Error while guessing a word.
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub enum GameGuessError {
    /// The guessed word length did not match the word to guess length.
    LengthInvalid {
        /// Length of the guessed word.
        given: usize,
        /// Length of the word to guess.
        expected: usize,
    },
    /// The submitted word has already been played before.
    AlreadyPlayed,
}

/// C wrapper to represent [Game].
#[repr(C)]
pub struct GameT {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// C wrapper to represent [GameState].
#[repr(C)]
pub enum GameStateT {
    /// The game has not ended.
    Pending = 0,
    /// The player guessed the target word.
    Won,
    /// The player ran out of guess attempts: they lost the game.
    Lost,
}

impl std::convert::From<GameState> for GameStateT {
    fn from(game_state: GameState) -> Self {
        match game_state {
            GameState::Pending {
                attempts_remaining: _,
            } => GameStateT::Pending,
            GameState::Won { attempts: _ } => GameStateT::Won,
            GameState::Lost => GameStateT::Lost,
        }
    }
}

/// C wrapper that constitutes a linked list.
///
/// See [Game::guess_hints(), wc_game_get_guess_hints].
#[repr(C)]
pub struct GuessHintListNodeT {
    current: *mut GuessHintT,
    next: *mut GuessHintListNodeT,
}

/// C wrapper to represent [GameGuessError].
#[repr(C)]
pub enum GameGuessErrorT {
    /// The submitted word length did not match the word to guess length.
    LengthInvalid = 0,
    /// The submitted word has already been played.
    AlreadyPlayed,
}

impl std::convert::From<GameGuessError> for GameGuessErrorT {
    fn from(error: GameGuessError) -> Self {
        match error {
            GameGuessError::LengthInvalid {
                given: _,
                expected: _,
            } => Self::LengthInvalid,
            GameGuessError::AlreadyPlayed => Self::AlreadyPlayed,
        }
    }
}

/// C wrapper to free a string allocated by rust.
///
/// # Safety
///
/// The C side should not modify the string length.
/// The pointer must have been allocated on the rust side.
#[no_mangle]
pub unsafe extern "C" fn rust_str_free(string: *mut std::os::raw::c_char) {
    let _ = std::ffi::CString::from_raw(string);
}

/// C wrapper to create a new game.
///
/// # Safety
///
/// `word_to_guess` must be a valid pointer to a `NULL`-terminated string.
///
/// Must be freed with [wc_game_free()].
#[no_mangle]
pub unsafe extern "C" fn wc_game_new(word_to_guess: *const std::os::raw::c_char) -> *mut GameT {
    let word_to_guess = std::ffi::CStr::from_ptr(word_to_guess);
    let new_game = Box::new(Game::new(&word_to_guess.to_string_lossy()).unwrap());
    Box::into_raw(new_game) as *mut GameT
}

/// C wrapper to create a new game with a specific attempts count limit.
///
/// # Safety
///
/// `word_to_guess` must be a valid pointer to a `NULL`-terminated string.
///
/// Must be freed with [wc_game_free()].
#[no_mangle]
pub unsafe extern "C" fn wc_game_new_with_attempts_count_limit(
    word_to_guess: *const std::os::raw::c_char,
    attempts_count_limit: u32,
) -> *mut GameT {
    let word_to_guess = std::ffi::CStr::from_ptr(word_to_guess);
    let new_game = Box::new(Game::new_with_attempts_count_limit(
        &word_to_guess.to_string_lossy(),
        attempts_count_limit as usize,
    ).unwrap());
    Box::into_raw(new_game) as *mut GameT
}

/// C wrapper to free memory allocated by [wc_game_new()] or
///
/// # Safety
///
/// `game`, if not `NULL` must point to a valid instance of `GameT`.
///
/// [wc_game_new_with_attempts_count_limit()].
#[no_mangle]
pub unsafe extern "C" fn wc_game_free(game: *mut GameT) {
    if !game.is_null() {
        let _ = Box::from_raw(game as *mut Game);
    }
}

/// C wrapper to get the word to guess.
///
/// The result must be freed by calling [rust_str_free()].
#[no_mangle]
pub extern "C" fn wc_game_get_word_to_guess(game: *const GameT) -> *mut std::os::raw::c_char {
    let game = {
        assert!(!game.is_null());
        unsafe { &*(game as *const Game) }
    };
    let word_to_guess = std::ffi::CString::new(game.word_to_guess()).unwrap();
    word_to_guess.into_raw()
}

/// C wrapper to retrieve the game state.
///
/// See [Game::state()].
#[no_mangle]
pub extern "C" fn wc_game_get_state(game: *const GameT) -> GameStateT {
    let game = {
        assert!(!game.is_null());
        unsafe { &*(game as *const Game) }
    };
    game.state().into()
}

/// C wrapper to get the current guess hint.
///
/// The result pointer must be freed with [wc_guess_hint_free].
///
/// See [Game::current_guess_hint()].
#[no_mangle]
pub extern "C" fn wc_game_get_current_guess_hint(game: *const GameT) -> *mut GuessHintT {
    let game = {
        assert!(!game.is_null());
        unsafe { &*(game as *const Game) }
    };
    match game.current_guess_hint() {
        Some(guess_hint) => Box::into_raw(Box::new(guess_hint)) as *mut GuessHintT,
        None => std::ptr::null_mut(),
    }
}

/// Free a guess hint
/// # Safety
/// `guess_hint` must not have been modified.
#[no_mangle]
pub unsafe extern "C" fn wc_guess_hint_free(guess_hint: *mut GuessHintT) {
    if !guess_hint.is_null() {
        let _ = Box::from_raw(guess_hint);
    }
}

/// C wrapper to get the list of guess hints.
///
/// The result is `NULL` if no guesses have yet been made.
/// Otherwise, it is a linked list.
///
/// Memory must be freed by calling [wc_game_guess_hints_free].
///
/// See [Game::guess_hints()].
#[no_mangle]
pub extern "C" fn wc_game_get_guess_hints(game: *const GameT) -> *mut GuessHintListNodeT {
    let game = {
        assert!(!game.is_null());
        unsafe { &*(game as *const Game) }
    };
    let mut guess_hints = game.guess_hints().collect::<Vec<GuessHint>>();
    if guess_hints.is_empty() {
        std::ptr::null_mut()
    } else {
        let mut previous_node = Option::<*mut GuessHintListNodeT>::None;
        let mut first = Option::<*mut GuessHintListNodeT>::None;
        while !guess_hints.is_empty() {
            let current_guess_hint =
                Box::into_raw(Box::new(guess_hints.remove(0))) as *mut GuessHintT;

            let current_node = Box::into_raw(Box::new(GuessHintListNodeT {
                current: current_guess_hint,
                next: std::ptr::null_mut(),
            })) as *mut GuessHintListNodeT;

            if let Some(previous) = previous_node {
                unsafe {
                    (*previous).next = current_node;
                }
            }

            previous_node = Some(current_node);
            if first.is_none() {
                first = Some(current_node);
            }
        }
        first.unwrap()
    }
}

/// # Safety
/// The linked will be freed.
/// The next item of each node should be left untouched.
///
/// See [wc_game_get_guess_hints].
#[no_mangle]
pub unsafe extern "C" fn wc_game_guess_hints_free(node: *mut GuessHintListNodeT) {
    if node.is_null() {
        return;
    }
    let mut to_free = Some(node);
    while let Some(current_to_free) = to_free {
        let current_to_free = Box::from_raw(current_to_free);
        let _ = Box::from_raw(current_to_free.current);
        to_free = if current_to_free.next.is_null() {
            None
        } else {
            Some(current_to_free.next)
        };
    }
}

/// C wrapper to make a game guess.
///
/// # Safety
/// `guessed_word` must be a `NULL`-terminated string.
///
/// See [Game::guess()].
#[no_mangle]
pub unsafe extern "C" fn wc_game_guess(
    game: *mut GameT,
    guessed_word: *const std::os::raw::c_char,
    error: *mut GameGuessErrorT,
    new_state: *mut GameStateT,
) -> bool {
    let game = {
        assert!(!game.is_null());
        &mut *(game as *mut Game)
    };
    let guessed_word = std::ffi::CStr::from_ptr(guessed_word);
    match game.guess(&guessed_word.to_string_lossy()) {
        Err(game_guess_error) => {
            if !error.is_null() {
                *error = game_guess_error.into();
            }
            false
        }
        Ok(new_game_state) => {
            if !new_state.is_null() {
                *new_state = new_game_state.into();
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, GameNewError, GameGuessError, GameState, GuessHint};

    #[test]
    fn game_new() {
        let game = Game::new("test").expect("new game");
        assert_eq!(&game.word_to_guess, "TEST");
        assert!(game.guesses.is_empty());
        assert_eq!(game.attempts_count_limit, 6);
    }

    #[test]
    fn game_new_empty() {
        assert_eq!(Game::new(""), Err(GameNewError::WordToGuessEmpty));
    }

    #[test]
    fn game_new_with_attempts_count_limit() {
        let game = Game::new_with_attempts_count_limit("test", 7).expect("new game");
        assert_eq!(&game.word_to_guess, "TEST");
        assert!(game.guesses.is_empty());
        assert_eq!(game.attempts_count_limit, 7);
    }

    #[test]
    fn game_new_with_attempts_count_limit_null() {
        assert_eq!(
            Game::new_with_attempts_count_limit("test", 0),
            Err(GameNewError::AttemptsCountLimitNull)
        );
    }

    #[test]
    fn game_state_pending() {
        let game = Game::new("test").expect("new game");
        assert_eq!(
            game.state(),
            GameState::Pending {
                attempts_remaining: 6
            }
        );
    }

    #[test]
    fn game_state_won() {
        let mut game = Game::new("temp").expect("new game");
        game.guesses.push(String::from("TEMP"));
        assert_eq!(game.state(), GameState::Won { attempts: 1 });
    }

    #[test]
    fn game_state_lost() {
        let mut game = Game::new("temp").expect("new game");
        game.guesses = vec![String::from("TEST"); 6];
        assert_eq!(game.state(), GameState::Lost);
    }

    #[test]
    fn game_current_guess_hint() {
        let mut game = Game::new("temp").expect("new game");
        assert!(game.current_guess_hint().is_none());

        game.guesses = vec![String::from("TEST"), String::from("THIS")];
        let mut guess_hints = game.guess_hints();
        assert_eq!(guess_hints.next(), Some(GuessHint::new("TEST", "TEMP").expect("new guess hint")));
        assert_eq!(guess_hints.next(), Some(GuessHint::new("THIS", "TEMP").expect("new guess hint")));
        assert_eq!(guess_hints.next(), None);
    }

    #[test]
    fn game_guess_invalid_length() {
        let mut game = Game::new("temp").expect("new game");
        assert_eq!(
            game.guess("it"),
            Err(GameGuessError::LengthInvalid {
                given: 2,
                expected: 4
            })
        );
    }

    #[test]
    fn game_guess_state() {
        let mut game = Game::new("temp").expect("new game");
        assert_eq!(
            game.guess("this"),
            Ok(GameState::Pending {
                attempts_remaining: 5
            })
        );
    }

    #[test]
    fn game_guess_already_played() {
        let mut game = Game::new("temp").expect("new game");
        assert_eq!(
            game.guess("this"),
            Ok(GameState::Pending {
                attempts_remaining: 5
            })
        );
        assert_eq!(game.guess("this"), Err(GameGuessError::AlreadyPlayed));
        assert_eq!(game.guess("This"), Err(GameGuessError::AlreadyPlayed));
    }

    #[test]
    fn game_get_word_to_guess() {
        let mut game = Game::new("temp").expect("new game");
        assert_eq!(game.word_to_guess(), "TEMP");
        game.word_to_guess = String::from("HELLO");
        assert_eq!(game.word_to_guess(), "HELLO");
    }
}
