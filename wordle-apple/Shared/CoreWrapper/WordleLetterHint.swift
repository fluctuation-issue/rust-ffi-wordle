import Foundation

public enum WordleLetterHint {
    case correct
    case placementIncorrect
    case incorrect

    init(wc_letter_hint: wc_letter_hint) {
        switch wc_letter_hint
        {
        case WC_LETTER_HINT_CORRECT:
            self = .correct
        case WC_LETTER_HINT_INCORRECT:
            self = .incorrect
        case WC_LETTER_HINT_PLACEMENT_INCORRECT:
            self = .placementIncorrect
        default:
            fatalError("invalid letter hint")
        }
    }
}

public struct WordleGuessedLetterAndHint {
    public let letter: Character
    public let hint: WordleLetterHint

    init(wc_guessed_letter_and_hint: wc_guessed_letter_and_hint) {
        letter = Character(UnicodeScalar(UInt8(wc_guessed_letter_and_hint.letter)))
        hint = WordleLetterHint(wc_letter_hint: wc_guessed_letter_and_hint.hint)
    }
}
