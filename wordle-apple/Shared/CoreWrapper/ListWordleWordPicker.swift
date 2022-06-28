import Foundation

public class ListWordleWordPicker: WordleWordPicker {
    private var innerWordPicker: wc_word_picker_t!

    public init(words: [String]) {
        var list = words.map { strdup($0) }
        list.append(nil)
        list.withUnsafeMutableBufferPointer { buf in
            innerWordPicker = wc_word_picker_new_from_list(buf.baseAddress)
        }
        list.forEach {
            free($0)
        }
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
