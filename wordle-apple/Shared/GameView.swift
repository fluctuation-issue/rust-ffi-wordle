import SwiftUI

struct GameView: View {
    @State private var guessed: String = ""
    @State private var displayGuessIsEmptyAlert = false
    @State private var displayGuessInvalidLengthAlert = false
    @State private var displayAlreadyPlayedAlert = false
    @ObservedObject var viewModel: GameViewModel

    var body: some View {
        switch viewModel.state {
        case .pending:
            pendingView
        case .won:
            wonView
        case .lost:
            lostView
        }
    }

    private var wonView: some View {
        VStack {
            WordleGuessesView(guesses: viewModel.game.guesses)
            GameWonSummaryView(attempts: Array(viewModel.game.guessHints).count)
            resetButton
        }.padding()
    }

    private var lostView: some View {
        VStack {
            WordleGuessesView(guesses: viewModel.game.guesses)
            GameLostSummaryView(wordToGuess: viewModel.game.wordToGuess)
            resetButton
        }.padding()
    }

    private var resetButton: some View {
        Button("Reset") {
            guessed = ""
            viewModel.reset()
        }
    }

    private var pendingView: some View {
        VStack {
            WordleGuessesView(guesses: viewModel.game.guesses)
            TextField("My guess", text: $guessed)
                .onSubmit {
                    attemptGuess()
                }
            Button("Submit") {
                attemptGuess()
            }
        }
        .alert("Guess Error", isPresented: $displayGuessIsEmptyAlert, actions: {}) {
            Text("You can not guess an empty word.")
        }
        .alert("Guess Error", isPresented: $displayGuessInvalidLengthAlert, actions: {}) {
            Text("Your word must be \(viewModel.game.wordToGuess.count)-character long. It was \(guessed.count).")
        }
        .alert("Guess Error", isPresented: $displayAlreadyPlayedAlert, actions: {}) {
            Text("This word was already guessed.")
        }
        .padding()
    }

    private func attemptGuess() {
        guard !guessed.isEmpty else {
            displayGuessIsEmptyAlert = true
            return
        }

        let guessResult = viewModel.guess(guessed)
        if let error = guessResult.error {
            switch error {
            case .alreadyPlayed:
                displayAlreadyPlayedAlert = true
            case .lengthInvalid:
                displayGuessInvalidLengthAlert = true
            }
        }
    }
}

struct GameView_Previews: PreviewProvider {
    static var previews: some View {
        GameView(viewModel: GameViewModel.previewGameViewModel)
    }
}
