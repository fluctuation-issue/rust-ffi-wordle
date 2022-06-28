import SwiftUI

struct GameWonSummaryView: View {
    let attempts: Int

    var body: some View {
        VStack {
            Text("Game Won :)")
                .font(.title)
                .foregroundColor(Constants.Appearance.wonColor)
            Text("Congratulations, you guessed the word in \(attempts) attempts.")
                .foregroundColor(Constants.Appearance.wonColor)
        }
    }
}

struct GameWonView_Previews: PreviewProvider {
    static var previews: some View {
        GameWonSummaryView(attempts: 2)
    }
}
