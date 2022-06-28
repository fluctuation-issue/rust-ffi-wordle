import SwiftUI

struct WordLengthHintView: View {
    var wordToGuessLength: Int

    var body: some View {
        VStack {
            Text("Your word is \(wordToGuessLength)-letter long.")
            GeometryReader { geometry in
                let size = geometry.size.width * (1 - Constants.Appearance.LetterCase.ROW_SPACING_PERCENT) / Double(wordToGuessLength)
                let spacing = geometry.size.width * Constants.Appearance.LetterCase.ROW_SPACING_PERCENT / Double(wordToGuessLength - 1)
                HStack(spacing: spacing) {
                    ForEach(0..<wordToGuessLength, id: \.self) { _ in
                        Rectangle()
                            .foregroundColor(Constants.Appearance.LetterCase.Colors.length)
                            .frame(width: size, height: size)
                    }
                }
            }
        }
    }
}

struct WordLengthHintView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            WordLengthHintView(wordToGuessLength: 5)
                .previewDisplayName("5-letter word")
            WordLengthHintView(wordToGuessLength: 6)
                .previewDisplayName("6-letter word")
            WordLengthHintView(wordToGuessLength: 10)
                .previewDisplayName("10-letter word")
        }
    }
}
