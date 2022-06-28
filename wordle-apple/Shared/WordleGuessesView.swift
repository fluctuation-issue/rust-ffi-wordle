import SwiftUI

struct WordleGuessesView: View {
    var guesses: WordleGame.Guesses

    private var lettersAndHints: [WordleGuessedLetterAndHint] {
        guesses.guessHints.flatMap {
            $0.guessedLettersAndHints
        }
    }

    var body: some View {
        if guesses.guessHints.isEmpty {
            WordLengthHintView(wordToGuessLength: guesses.targetWorld.count)
        } else {
            let wordToGuessLength = guesses.guessHints[0].guessed.count
            let columns = Array(repeating: GridItem(.flexible()), count: wordToGuessLength)
            GeometryReader { geometry in
                let size = geometry.size.width * (1 - Constants.Appearance.LetterCase.ROW_SPACING_PERCENT) / Double(wordToGuessLength)

                let lettersAndHints = self.lettersAndHints

                LazyVGrid(columns: columns) {
                    ForEach(lettersAndHints.indices, id: \.self) {
                        index in
                        let letterAndHint = lettersAndHints[index]
                        Text("\(letterAndHint.letter.uppercased())")
                            .frame(width: size, height: size)
                            .background(Constants.Appearance.LetterCase.Colors.forLetterHint(letterAndHint.hint))
                    }
                }
            }
        }
    }
}

struct WordleGuessesView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            WordleGuessesView(guesses: WordleGame.Guesses(targetWorld: "wordle-swift-rust", guessHints: []))
                .previewDisplayName("No guesses yet")
        }
    }
}
