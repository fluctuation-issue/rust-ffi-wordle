import SwiftUI

@main
struct WordleAppleApp: App {
    private var wordsFilePath: String = {
        return try! FileManager.default.url(for: .documentDirectory, in: .userDomainMask, appropriateFor: nil, create: false).appendingPathComponent("wordle-words.txt").path
    }()

    var body: some Scene {
        WindowGroup {
            ContentView(fileWordPicker: FileWordPicker(path: wordsFilePath))
        }
    }
}
