import SwiftUI

struct ContentView: View {
    var fileWordPicker: FileWordPicker

    private var gameViewModel: GameViewModel {
        GameViewModel(picker: fileWordPicker)
    }

    private var settingsViewModel: SettingsViewModel {
        SettingsViewModel(fileWordPicker: fileWordPicker)
    }

    var body: some View {
        TabView {
            GameView(viewModel: gameViewModel).tabItem {
                Image(systemName: "dice")
                Text("Game")
            }.tag(0)
            SettingsView(settingsViewModel: settingsViewModel).tabItem {
                Image(systemName: "gearshape")
                Text("Settings")
            }.tag(1)
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            ContentView(fileWordPicker: FileWordPicker(path: ""))
            ContentView(fileWordPicker: FileWordPicker(path: ""))
                .preferredColorScheme(.dark)
        }
    }
}
