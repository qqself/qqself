import {
  createApiFindRequest,
  createNewKeys,
} from "../core/pkg/qqself_client_web_core";

type ResponseError = {error_code: number, error: string}

describe("API", () => {

  test("Create new keys", async () => {
    const keys = createNewKeys();
    expect(keys.publicKey).toBeTruthy();
    expect(keys.privateKey).toBeTruthy();
  });

  test("Find endpoint", async () => {
    const keys = createNewKeys();
    const req = createApiFindRequest(keys);
    const resp = await fetch(req.url, {
      method: "POST",
      body: req.payload,
      headers: {
        "Content-Type": req.contentType,
      },
    });
    expect(resp.status).toBe(200)
  });
});
