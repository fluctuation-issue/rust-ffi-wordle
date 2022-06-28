import Foundation
import SwiftUI

public struct Constants {
    public struct Appearance {
        public struct LetterCase {
            public static let ROW_SPACING_PERCENT = 0.2

            public struct Colors {
                public static let length = Color.gray

                public static func forLetterHint(_ letterHint: WordleLetterHint) -> Color {
                    switch letterHint {
                    case .correct:
                        return Color.green
                    case .incorrect:
                        return Color.gray
                    case .placementIncorrect:
                        return Color.orange
                    }
                }
            }
        }

        public static let lostColor = Color.red
        public static let wonColor = Color.green
    }

    public static let defaultWords = [
        "wordle",
        "wordlerust",
    ]
}
