import SwiftUI
import qqselfCoreLib

struct MainView: View {
    let hash = stringHash(input: "Hello")
    var body: some View {
        TabView(selection: .constant(1),
                content:  {
            WeekView().tabItem {
                Tab(image: "icon_week", text: "Week")
            }.tag(1)
            JournalView().tabItem {
                Tab(image:"icon_journal", text: "Journal")
            }.tag(2)
            SkillsView().tabItem {
                Tab(image:"icon_id", text:"Identities")
            }.tag(3)
            RejectionsView().tabItem {
                Tab(image: "icon_rejection", text: "Rejections")
            }.tag(4)
        })
    }
}

struct Tab: View {
    let image: String
    let text: String
    var body: some View {
        VStack {
            Image(self.image)
            Text(self.text)
        }
    }
}

#Preview {
    MainView()
}
