import Foundation

public class WordleGame {
    public struct Guesses {
        let targetWorld: String
        let guessHints: [WordleGuessHint]
    }

    public struct GuessResult {
        let error: GuessError?
        let state: State
    }

    public enum State {
        case won
        case lost
        case pending

        init(wc_game_state: wc_game_state) {
            if wc_game_state == WC_GAME_STATE_LOST {
                self = .lost
            } else if wc_game_state == WC_GAME_STATE_PENDING {
                self = .pending
            } else if wc_game_state == WC_GAME_STATE_WON {
                self = .won
            } else {
                fatalError("unimplemented wc game state")
            }
        }
    }

    public enum GuessError {
        case lengthInvalid
        case alreadyPlayed

        init(wc_game_guess_error: wc_game_guess_error) {
            if wc_game_guess_error == WC_GAME_GUESS_ERROR_ALREADY_PLAYED {
                self = .alreadyPlayed
            } else if wc_game_guess_error == WC_GAME_GUESS_ERROR_LENGTH_INVALID {
                self = .lengthInvalid
            } else {
                fatalError("unimplemented wc game guess error")
            }
        }
    }

    private var innerGame: wc_game_t

    public init(wordToGuess: String) {
        innerGame = wc_game_new(wordToGuess)
    }

    public init(wordToGuess: String, attemptsLimit: UInt32) {
        innerGame = wc_game_new_with_attempts_count_limit(wordToGuess, attemptsLimit)
    }

    deinit {
        wc_game_free(innerGame)
    }

    public var wordToGuess: String {
        let cString = wc_game_get_word_to_guess(innerGame)
        let result = String(cString: UnsafeMutablePointer(mutating: cString!))
        rust_str_free(cString)
        return result
    }

    public var currentGuessHint: WordleGuessHint? {
        let guessHint = wc_game_get_current_guess_hint(innerGame)
        return guessHint == nil ? nil : WordleGuessHint(guessHint: guessHint!, mustFree: true)
    }

    public var guessHints: WordleGuessHints {
        WordleGuessHints(guessHintNode: wc_game_get_guess_hints(innerGame))
    }

    public var state: State {
        let wc_game_state = wc_game_get_state(innerGame)
        return State(wc_game_state: wc_game_state)
    }

    public var guesses: Guesses {
        return Guesses(targetWorld: wordToGuess, guessHints: Array(guessHints))
    }

    public func guess(guessed: String) -> GuessResult {
        var errorCode: wc_game_guess_error = WC_GAME_GUESS_ERROR_ALREADY_PLAYED
        var state: wc_game_state = WC_GAME_STATE_LOST
        let anErrorOccurred = wc_game_guess(innerGame, guessed, &errorCode, &state)
        let error = anErrorOccurred != 1 ? GuessError(wc_game_guess_error: errorCode) : nil
        return GuessResult(error: error, state: State(wc_game_state: state))
    }
}
