import Foundation

class SettingsViewModel: ObservableObject
{
    @Published var words: [String]
    var fileWordPicker: FileWordPicker

    init(fileWordPicker: FileWordPicker) {
        self.fileWordPicker = fileWordPicker
        words = fileWordPicker.words
    }

    func addWord(_ word: String) {
        fileWordPicker.addWord(word)
        words = fileWordPicker.words
    }

    func removeWords(atOffsets offsets: IndexSet) {
        fileWordPicker.removeWords(atOffsets: offsets)
        words = fileWordPicker.words
    }
}
