import SwiftUI

struct GameLostSummaryView: View {
    let wordToGuess: String

    var body: some View {
        VStack {
            Text("Game Lost :(")
                .font(.title)
                .foregroundColor(Constants.Appearance.lostColor)
            Text("The word to guess was: \(wordToGuess.uppercased()).")
                .foregroundColor(Constants.Appearance.lostColor)
        }
    }
}

struct GameLostView_Previews: PreviewProvider {
    static var previews: some View {
        GameLostSummaryView(wordToGuess: "banana")
    }
}
