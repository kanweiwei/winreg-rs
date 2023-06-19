import test from 'ava'

import { JsRegistry } from '../index'

test('sync function from native code', (t) => {
  new JsRegistry('HKCU');
  t.pass();
})
