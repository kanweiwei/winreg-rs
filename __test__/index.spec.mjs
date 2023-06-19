import test from "ava";

import { JsRegistry, HKCU } from "../index.js";

test("sum from native", (t) => {
  new JsRegistry(HKCU);
  t.pass();
});
