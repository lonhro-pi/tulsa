import Foundation

@MainActor
final class TerminalViewModel: ObservableObject {
    @Published var output: String = ""
    @Published var input: String = ""
    @Published var serverUrl: String = "ws://HOST:7070/ws"
    @Published var token: String = ""
    @Published var connected: Bool = false
    @Published var status: String = "Disconnected"

    private var webSocket: URLSessionWebSocketTask?

    func toggleConnection() {
        if connected {
            disconnect()
        } else {
            connect()
        }
    }

    func connect() {
        guard let url = URL(string: serverUrl) else {
            status = "Invalid URL"
            return
        }
        var request = URLRequest(url: url)
        if !token.isEmpty {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }
        let task = URLSession.shared.webSocketTask(with: request)
        task.resume()
        webSocket = task
        connected = true
        status = "Connected"
        receive()
    }

    func disconnect() {
        webSocket?.cancel(with: .goingAway, reason: nil)
        webSocket = nil
        connected = false
        status = "Disconnected"
    }

    func sendLine() {
        let line = input + "\n"
        input = ""
        send(text: line)
    }

    func send(text: String) {
        guard let webSocket else { return }
        webSocket.send(.string(text)) { [weak self] error in
            if let error {
                DispatchQueue.main.async {
                    self?.status = "Send error: \(error.localizedDescription)"
                    self?.connected = false
                }
            }
        }
    }

    private func receive() {
        guard let webSocket else { return }
        webSocket.receive { [weak self] result in
            guard let self else { return }
            switch result {
            case .success(let message):
                switch message {
                case .string(let text):
                    self.appendOutput(text)
                case .data(let data):
                    let text = String(decoding: data, as: UTF8.self)
                    self.appendOutput(text)
                @unknown default:
                    break
                }
                self.receive()
            case .failure(let error):
                DispatchQueue.main.async {
                    self.status = "Receive error: \(error.localizedDescription)"
                    self.connected = false
                }
            }
        }
    }

    private func appendOutput(_ text: String) {
        output.append(text)
        trimOutput()
    }

    private func trimOutput() {
        let limit = 200_000
        if output.count > limit {
            output = String(output.suffix(limit))
        }
    }
}
