import Foundation

class FileWordleWordPicker: WordleWordPicker {
    private var innerWordPicker: wc_word_picker_t!

    public init(filePath: String) {
        var path = filePath.cString(using: .utf8)!
        innerWordPicker = wc_word_picker_new_random_line_file(&path)
    }

    deinit {
        wc_word_picker_free(innerWordPicker)
    }

    public func pickWord() -> String {
        let cString = wc_word_picker_pick_word(innerWordPicker)
        let result = String(cString: cString!)
        rust_str_free(cString)
        return result
    }
}
