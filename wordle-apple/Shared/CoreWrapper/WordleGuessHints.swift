import Foundation

public class WordleGuessHints: Sequence {
    public struct HintIterator: IteratorProtocol {
        private var node: UnsafeMutablePointer<wc_guess_hint_list_node_t>?

        init(node: UnsafeMutablePointer<wc_guess_hint_list_node_t>?) {
            self.node = node
        }

        mutating public func next() -> WordleGuessHint? {
            guard let node = node else {
                return nil
            }
            let currentGuessHint = WordleGuessHint(guessHint: node.pointee.current, mustFree: false)
            self.node = node.pointee.next
            return currentGuessHint
        }
    }

    private var innerGuessHintNode: UnsafeMutablePointer<wc_guess_hint_list_node_t>?

    init(guessHintNode: UnsafeMutablePointer<wc_guess_hint_list_node_t>?) {
        innerGuessHintNode = guessHintNode
    }

    deinit {
        if let node = innerGuessHintNode {
            wc_game_guess_hints_free(node)
        }
    }

    public func makeIterator() -> HintIterator {
        HintIterator(node: innerGuessHintNode)
    }
}

