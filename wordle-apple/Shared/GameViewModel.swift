import Foundation

class GameViewModel: ObservableObject {
    @Published private(set) var state: WordleGame.State = .pending

    var picker: WordleWordPicker
    private(set) var game: WordleGame

    init(picker: WordleWordPicker) {
        self.picker = picker
        self.game = WordleGame(wordToGuess: picker.pickWord())
    }

    func reset() {
        let picked = picker.pickWord()
        self.game = WordleGame(wordToGuess: picked)
        state = .pending
    }

    func guess(_ guessed: String) -> WordleGame.GuessResult {
        let result = game.guess(guessed: guessed)
        state = game.state
        return result
    }
}

#if DEBUG
extension GameViewModel {
    static var previewGameViewModel: GameViewModel = {
        let picker = ListWordleWordPicker(words: Constants.defaultWords)
        return GameViewModel(picker: picker)
    }()
}
#endif
