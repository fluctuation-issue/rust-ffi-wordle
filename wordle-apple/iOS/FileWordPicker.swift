import Foundation

class FileWordPicker {
    private var filePath: String
    private var fileWords: [String]

    var words: [String] {
        fileWords
    }

    init(path: String) {
        filePath = path
        if let data = try? Data(contentsOf: URL(fileURLWithPath: path)), let string = String(data: data, encoding: .utf8) {
            let words = string.split(separator: "\n")
            if words.isEmpty {
                fileWords = Constants.defaultWords
            } else {
                fileWords = words.map(String.init)
            }
        } else {
            fileWords = Constants.defaultWords
        }
        fileWords = fileWords.filter { !$0.isEmpty }.map { $0.uppercased() }
        fileWords.sort()
    }

    func addWord(_ word: String) {
        let wordToAdd = word.uppercased()
        guard !fileWords.contains(wordToAdd) else { return }
        let newIndex = findWordInsertionPoint(wordToAdd)
        fileWords.insert(wordToAdd, at: newIndex)
        writeWords()
    }

    func removeWords(atOffsets offsets: IndexSet) {
        self.fileWords.remove(atOffsets: offsets)
        writeWords()
    }

    private func findWordInsertionPoint(_ newElement: String) -> Int {
      var startIndex = 0
      var endIndex = fileWords.count

      while startIndex < endIndex {
          let midIndex = startIndex + (endIndex - startIndex) / 2
          if fileWords[midIndex] == newElement {
              return midIndex
          } else if fileWords[midIndex] < newElement {
              startIndex = midIndex + 1
          } else {
              endIndex = midIndex
          }
      }
      return startIndex
    }

    private func writeWords() {
        let wordsContent = fileWords.joined(separator: "\n")
        let wordsData = wordsContent.data(using: .utf8)
        FileManager.default.createFile(atPath: filePath, contents: wordsData)
    }
}

extension FileWordPicker: WordleWordPicker {
    func pickWord() -> String {
        fileWords.randomElement()!
    }
}
