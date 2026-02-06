import SwiftUI

struct ContentView: View {
    @StateObject private var model = TerminalViewModel()
    private let background = Color(red: 0.03, green: 0.03, blue: 0.05)
    private let accent = Color(red: 1.0, green: 0.0, blue: 0.5)
    private let accentAlt = Color(red: 0.0, green: 1.0, blue: 0.8)

    var body: some View {
        VStack(spacing: 12) {
            header
            connectionPanel
            outputPanel
            inputPanel
        }
        .padding()
        .background(background.ignoresSafeArea())
        .foregroundColor(.white)
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("iLonhro Terminal")
                .font(.title2)
                .foregroundColor(accent)
            Text("iLonhro by Lonhro")
                .font(.subheadline)
                .foregroundColor(accentAlt)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var connectionPanel: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Server")
                .font(.headline)
                .foregroundColor(accentAlt)
            TextField("ws://HOST:7070/ws", text: $model.serverUrl)
                .textInputAutocapitalization(.never)
                .autocorrectionDisabled(true)
                .padding(8)
                .background(Color.black.opacity(0.4))
                .cornerRadius(8)
            SecureField("Token", text: $model.token)
                .textInputAutocapitalization(.never)
                .autocorrectionDisabled(true)
                .padding(8)
                .background(Color.black.opacity(0.4))
                .cornerRadius(8)
            HStack {
                Button(model.connected ? "Disconnect" : "Connect") {
                    model.toggleConnection()
                }
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(accent)
                .cornerRadius(8)
                Text(model.status)
                    .font(.caption)
                    .foregroundColor(accentAlt)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var outputPanel: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Session")
                .font(.headline)
                .foregroundColor(accentAlt)
            ScrollView {
                Text(model.output)
                    .font(.system(.body, design: .monospaced))
                    .frame(maxWidth: .infinity, alignment: .topLeading)
                    .padding(8)
            }
            .background(Color.black.opacity(0.4))
            .cornerRadius(8)
        }
    }

    private var inputPanel: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Input")
                .font(.headline)
                .foregroundColor(accentAlt)
            HStack {
                TextField("Type a command", text: $model.input)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled(true)
                    .onSubmit {
                        model.sendLine()
                    }
                    .padding(8)
                    .background(Color.black.opacity(0.4))
                    .cornerRadius(8)
                Button("Send") {
                    model.sendLine()
                }
                .padding(.horizontal, 12)
                .padding(.vertical, 8)
                .background(accent)
                .cornerRadius(8)
            }
        }
    }
}
