import { html, render } from "lit"
import { OfflineApi, TestStore } from "../../utilsTests"
import { describe, test } from "vitest"
import { DateDay } from "../../../bridge/pkg"
import "./progress"

describe("progress", () => {
  test("render", async () => {
    const store = new TestStore(new OfflineApi())
    await store.dispatch("init.started", null)
    await store.dispatch("auth.registration.started", { mode: "automatic" })
    render(
      html`<q-progress-page
        .store=${store}
        .currentDay=${DateDay.fromDate(new Date("2022-06-07"))}
      />`,
      document.body
    )
    // Check that adding an entry doesn't cause any errors
    await store.dispatch("data.entry.added", {
      entry: "2022-06-07 10:00 11:00 foo",
      callSyncAfter: true,
    })
  })
})
