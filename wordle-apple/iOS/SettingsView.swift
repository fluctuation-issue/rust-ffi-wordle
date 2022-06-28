import SwiftUI

struct SettingsView: View {
    @State private var showNewWordAlert = false
    @State private var showNewWordEmptyError = false
    @State private var showNewWordAlreadyExistsError = false
    @State private var newWord = ""
    @ObservedObject var settingsViewModel: SettingsViewModel

    var body: some View {
        NavigationView {
            VStack {
                if settingsViewModel.words.isEmpty {
                    Text("No words", tableName: "Settings")
                } else {
                    List {
                        ForEach(settingsViewModel.words.indices, id: \.self) {
                            index in
                            let word = settingsViewModel.words[index]
                            Text(word)
                        }
                        .onDelete(perform: deleteWords)
                    }
                }
            }
                .navigationTitle(String(localized: "Words", table: "Settings"))
                .toolbar {
                    Button(role: .none, action: {
                        newWord = ""
                        showNewWordAlert = true
                    }, label: {
                        Image(systemName: "plus")
                    })
                }
                .textFieldAlert(isPresented: $showNewWordAlert, title: String(localized: "New Word", table: "Settings"), text: $newWord, placeholder: String(localized: "new word", table: "Settings"), action: {
                    newWord in
                    attemptToAddWord(newWord.uppercased())
                })
                .alert(String(localized: "New Word Error", table: "Settings"), isPresented: $showNewWordEmptyError, actions: {}, message: { Text("New word can not be empty.", tableName: "Settings") })
                .alert(String(localized: "New Word Error", table: "Settings"), isPresented: $showNewWordAlreadyExistsError, actions: {}, message: { Text("This word already exists.", tableName: "Settings") })
            }
    }

    private func deleteWords(at offsets: IndexSet) {
        settingsViewModel.removeWords(atOffsets: offsets)
    }

    private func attemptToAddWord(_ word: String) {
        if word.isEmpty {
           showNewWordEmptyError = true
        } else if settingsViewModel.words.contains(word) {
            showNewWordAlreadyExistsError = true
        } else {
            settingsViewModel.addWord(word)
        }
    }
}

struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView(settingsViewModel: SettingsViewModel(fileWordPicker: FileWordPicker(path: "")))
    }
}
