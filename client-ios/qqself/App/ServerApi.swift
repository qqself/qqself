import Foundation
import qqselfCoreLib

struct Response {
  let data: Data
  let urlResponse: HTTPURLResponse
}

// swift-format-ignore
struct ServerError: Codable {
  let error_code: Int16
  let error: String
}

enum ApiError: Error {
  case networkError
  case serverError(ServerError)
}

struct EncryptedEntry: Codable {
  let id: String
  let payload: String
}

protocol APIProvider {
  func set(payload: String) async throws -> String
  func find(payload: String) async throws -> [EncryptedEntry]
  func deleteAccount(payload: String) async throws
}

class ServerApi: APIProvider {
  let api: Api

  init(basePath: String?) {
    self.api = Api(basePath: basePath)
  }

  func makeHttpRequest(_ req: URLRequest) async throws -> (Data, URLResponse) {
    do {
      return try await URLSession.shared.data(for: req)
    } catch {
      throw ApiError.networkError
    }
  }

  func http(req: Request) async throws -> Response {
    let url = URL(string: req.url)!
    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.httpBody = req.payload.data(using: .utf8)
    for header in req.headers {
      request.setValue(header.value, forHTTPHeaderField: header.name)
    }

    let (data, response) = try await makeHttpRequest(request)
    guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
      let err = try JSONDecoder().decode(ServerError.self, from: data)
      throw ApiError.serverError(err)
    }
    return Response(data: data, urlResponse: httpResponse)
  }

  func set(payload: String) async throws -> String {
    let req = self.api.createSetRequest(payload: payload)
    let resp = try await http(req: req)
    return String(data: resp.data, encoding: .utf8)!
  }

  func find(payload: String) async throws -> [EncryptedEntry] {
    let req = api.createFindRequest(payload: payload)
    let resp = try await http(req: req)
    let body: String! = String(data: resp.data, encoding: .utf8)
    let lines = body.components(separatedBy: "\n").filter { !$0.isEmpty }
    return lines.map { entry in
      let components = entry.components(separatedBy: ":")
      return EncryptedEntry(id: components[0], payload: components[1])
    }
  }

  func deleteAccount(payload: String) async throws {
    let req = self.api.createDeleteRequest(payload: payload)
    _ = try await http(req: req)
  }
}
