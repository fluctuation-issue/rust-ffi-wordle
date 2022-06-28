import SwiftUI

@main
struct WordleAppleApp: App {
    var picker: WordleWordPicker {
        if CommandLine.arguments.count > 1 && FileManager.default.fileExists(atPath: CommandLine.arguments[1]) {
            return FileWordleWordPicker(filePath: CommandLine.arguments[1])
        } else {
            return ListWordleWordPicker(words: Constants.defaultWords)
        }
    }

    var viewModel: GameViewModel {
        return GameViewModel(picker: picker)
    }

    var body: some Scene {
        WindowGroup {
            ContentView(gameViewModel: viewModel)
        }
    }
}
