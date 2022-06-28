use std::io::{BufRead, Write};

use wordle_core::game::{Game, GameGuessError, GameState};
use wordle_core::hint::{GuessHint, LetterHint};
use wordle_core::word_pick::{RandomWordPicker, RandomWordPickerError, WordPicker};

use crate::ansi;
use crate::cli_arguments::{WordleCliExecutionError, WordleCliInput};

const WELCOME_SCREEN_SLEEP_MILLIS: u64 = 700;
const GOODBYE_SCREEN_SLEEP_MILLIS: u64 = 800;

pub fn run_game(input: WordleCliInput) -> Result<(), WordleCliExecutionError> {
    let random_word_chooser = initialize_random_word_chooser(input)?;
    play_games(random_word_chooser);
    Ok(())
}

fn initialize_random_word_chooser(
    input: WordleCliInput,
) -> Result<RandomWordPicker, WordleCliExecutionError> {
    let reader = input.into_reader().map_err(WordleCliExecutionError::Io)?;
    RandomWordPicker::from_reader(reader).map_err(|error| match error {
        RandomWordPickerError::Io(error) => WordleCliExecutionError::Io(error),
        RandomWordPickerError::NoWords => WordleCliExecutionError::NoWords,
    })
}

fn play_games<P: WordPicker>(picker: P) {
    ansi::switch_to_alternate_screen();
    print_welcome_screen();
    run_games_loop(picker);
    print_goodbye_screen();
    ansi::switch_from_alternate_screen();
}

fn print_welcome_screen() {
    println!("Welcome to WORDLE");
    std::thread::sleep(std::time::Duration::from_millis(
        WELCOME_SCREEN_SLEEP_MILLIS,
    ));
}

fn print_goodbye_screen() {
    ansi::clear_screen();
    println!("Thanks for playing WORDLE.\n\nSee you soon!");
    std::thread::sleep(std::time::Duration::from_millis(
        GOODBYE_SCREEN_SLEEP_MILLIS,
    ));
}

fn run_games_loop<P: WordPicker>(mut picker: P) {
    let mut playing = true;
    while playing {
        play_one_game(&mut picker);
        playing = ask_keep_playing();
    }
}

fn play_one_game<P: WordPicker>(picker: &mut P) {
    print_game_start_screen();

    let word_to_guess = picker.pick_word();
    let mut game = Game::new(&word_to_guess);
    let mut game_ended = false;
    while !game_ended {
        print_hints(&game);
        game_ended = try_to_guess_word(&mut game);
    }
}

fn print_game_start_screen() {
    ansi::clear_screen();
    println!("Playing one game of wordle");
}

/// Return whether the game has ended.
fn try_to_guess_word(game: &mut Game) -> bool {
    match guess_word(game) {
        Err(game_error) => print_game_guess_error(&game_error),
        Ok(game_state) => {
            ansi::clear_screen();
            match game_state {
                GameState::Lost => {
                    print_game_lost(game);
                    return true;
                }
                GameState::Won { attempts } => {
                    print_game_won(game, attempts);
                    return true;
                }
                GameState::Pending { attempts_remaining } => print_game_pending(attempts_remaining),
            }
        }
    }
    false
}

fn print_game_lost(game: &Game) {
    print_hints(game);
    println!(
        "You lost :(\nThe word to guess was {}.",
        game.word_to_guess()
    );
}

fn print_game_won(game: &Game, attempts: usize) {
    print_hints(game);
    println!(
        "You win with {} {} :)",
        attempts,
        get_attempts_text(attempts)
    );
}

fn print_game_pending(attempts_remaining: usize) {
    println!(
        "{} {} remaining",
        attempts_remaining,
        get_attempts_text(attempts_remaining)
    );
}

fn print_game_guess_error(game_error: &GameGuessError) {
    match game_error {
        GameGuessError::AlreadyPlayed => eprintln!("this word has already been played"),
        GameGuessError::LengthInvalid { given, expected } => eprintln!(
            "submitted word has invalid length {}: expected {}",
            given, expected
        ),
    }
}

fn print_hints(game: &Game) {
    if game.current_guess_hint().is_none() {
        print_word_to_guess_placeholder(game.word_to_guess());
    } else {
        print_guess_hints(game);
    }
}

fn print_word_to_guess_placeholder(word_to_guess: &str) {
    println!(
        "{} ({} characters)",
        "-".repeat(word_to_guess.len()),
        word_to_guess.len()
    );
}

fn print_guess_hints(game: &Game) {
    for hint in game.guess_hints() {
        print_guess_hint(hint);
    }
}

fn print_guess_hint(hint: GuessHint) {
    let hints = hint
        .guessed_letters_and_hints()
        .into_iter()
        .map(|(letter, hint)| format_guess_hint_letter(letter, hint))
        .collect::<Vec<String>>();
    println!("{}", hints.join(" "));
    println!();
}

fn format_guess_hint_letter(letter: char, hint: LetterHint) -> String {
    match hint {
        LetterHint::Correct => ansi::format_green_bg(letter),
        LetterHint::PlacementIncorrect => ansi::format_yellow_bg(letter),
        LetterHint::Incorrect => format!("{}", letter),
    }
}

enum GuessWordError {
    AlreadyPlayed,
    Empty,
    LengthInvalid { expected: usize },
}

fn get_guess_word_error(game: &Game, guessed: &str) -> Option<GuessWordError> {
    if guessed.is_empty() {
        Some(GuessWordError::Empty)
    } else if !word_has_correct_length(game, guessed) {
        Some(GuessWordError::LengthInvalid {
            expected: game.word_to_guess().len(),
        })
    } else if word_has_been_played(game, guessed) {
        Some(GuessWordError::AlreadyPlayed)
    } else {
        None
    }
}

fn guess_word(game: &mut Game) -> Result<GameState, GameGuessError> {
    let mut guess = String::new();
    let mut tty_stdin = get_tty_input();
    let mut guess_word_error;
    let mut first_error = true;
    while {
        print!("Your guess: ");
        std::io::stdout().flush().unwrap();
        guess.clear();
        tty_stdin
            .read_line(&mut guess)
            .expect("could not read guess line");
        guess = guess.trim().to_uppercase();

        guess_word_error = get_guess_word_error(game, &guess);
        match guess_word_error {
            Some(error) => {
                print_guess_word_error(error, first_error);
                first_error = false;
                true
            }
            None => false,
        }
    } {}
    game.guess(&guess)
}

fn print_guess_word_error(guess_word_error: GuessWordError, first_error: bool) {
    if !first_error {
        ansi::move_cursor_up(3);
    } else {
        ansi::move_cursor_up(1);
    }
    ansi::clear_to_end_of_screen();

    println!();
    match guess_word_error {
        GuessWordError::Empty => println!("Please provided a guess word."),
        GuessWordError::AlreadyPlayed => println!("This word has already been played."),
        GuessWordError::LengthInvalid { expected } => {
            println!("Please type a {}-letter word.", expected)
        }
    }
}

fn word_has_correct_length(game: &Game, guess: &str) -> bool {
    game.word_to_guess().len() == guess.len()
}

fn word_has_been_played(game: &Game, guess: &str) -> bool {
    game.guess_hints().any(|hint| hint.guessed() == guess)
}

fn ask_keep_playing() -> bool {
    let mut stdout = std::io::stdout();
    let mut tty_stdin = get_tty_input();
    loop {
        let mut buffer = String::new();
        print!("Do you want to keep playing? (y/n) ");
        let _ = stdout.flush();
        if tty_stdin.read_line(&mut buffer).is_err() {
            return false;
        }
        buffer = buffer.to_uppercase();
        match buffer.trim() {
            "Y" | "YES" => return true,
            "N" | "NO" => return false,
            _ => (),
        }
    }
}

fn get_tty_input() -> std::io::BufReader<impl std::io::Read> {
    std::io::BufReader::new(std::fs::File::open("/dev/tty").expect("unable to open tty"))
}

/// Return `"attempt"` with correct plural form.
fn get_attempts_text(n: usize) -> &'static str {
    if n == 1 {
        "attempt"
    } else {
        "attempts"
    }
}

#[cfg(test)]
mod tests {
    use super::get_attempts_text;

    #[test]
    fn get_attempts_text_singular() {
        assert_eq!(get_attempts_text(1), "attempt");
    }

    #[test]
    fn get_attempts_text_plural() {
        assert_eq!(get_attempts_text(0), "attempts");
        assert_eq!(get_attempts_text(2), "attempts");
        assert_eq!(get_attempts_text(3), "attempts");
        assert_eq!(get_attempts_text(10), "attempts");
    }
}
