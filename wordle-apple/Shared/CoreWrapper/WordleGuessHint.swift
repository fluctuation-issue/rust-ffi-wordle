import Foundation

public class WordleGuessHint {
    private let mustFree: Bool
    private let innerGuessHint: wc_guess_hint_t

    init(guessHint: wc_guess_hint_t, mustFree: Bool) {
        self.mustFree = mustFree
        self.innerGuessHint = guessHint
    }

    deinit {
        if mustFree {
            wc_guess_hint_free(innerGuessHint)
        }
    }

    var guessed: String {
        let cStringGuessed = wc_guess_hint_get_guessed(innerGuessHint)
        let result = String(cString: cStringGuessed!)
        rust_str_free(cStringGuessed)
        return result
    }

    var letterHints: [WordleLetterHint] {
        var result = [WordleLetterHint]()
        let letterHints = wc_guess_hint_get_letter_hints(innerGuessHint)!
        for i in 0..<letterHints.pointee.num_hints {
            let hint = letterHints.pointee.hints.advanced(by: Int(i)).pointee
            let letterHint = WordleLetterHint(wc_letter_hint: hint)
            result.append(letterHint)
        }
        wc_guess_hint_free_letter_hints(letterHints)
        return result
    }

    var guessedLettersAndHints: [WordleGuessedLetterAndHint] {
        var result = [WordleGuessedLetterAndHint]()
        let guessedLettersAndHints = wc_guess_hint_get_guessed_letters_and_hints(innerGuessHint)!
        for i in 0..<guessedLettersAndHints.pointee.num_letters_and_hints {
            let hint = guessedLettersAndHints.pointee.hints.advanced(by: Int(i)).pointee
            let letterHint = WordleGuessedLetterAndHint(wc_guessed_letter_and_hint: hint)
            result.append(letterHint)
        }
        wc_guess_hint_free_guessed_letters_and_hints(guessedLettersAndHints)
        return result
    }
}
