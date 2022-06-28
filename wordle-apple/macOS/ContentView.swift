import SwiftUI

struct ContentView: View {
    @ObservedObject var gameViewModel: GameViewModel

    var body: some View {
        GameView(viewModel: gameViewModel)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(gameViewModel: GameViewModel.previewGameViewModel)
    }
}
