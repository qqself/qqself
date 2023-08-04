import "./progress"

import { html, render } from "lit"
import { describe, expect, test } from "vitest"

import { DateDay } from "../../../bridge/pkg"
import { OfflineApi, TestStore } from "../../utilsTests"

describe("progress", () => {
  test("render", async () => {
    const store = new TestStore(expect, new OfflineApi())
    await store.dispatch("init.started", null)
    await store.dispatch("auth.registration.started", { mode: "automatic" })
    render(
      html`<q-progress-page
        .store=${store}
        .currentDay=${DateDay.fromDate(new Date("2022-06-07"))}
      />`,
      document.body,
    )
    // Check that adding an entry doesn't cause any errors
    await store.dispatch("data.entry.added", {
      entry: "2022-06-07 10:00 11:00 foo",
      callSyncAfter: true,
    })
  })
})
