import * as Path from 'path'
import { formatWithOptions } from 'util'

import test from 'ava'

import { exec } from '../index'

const cwd = Path.join(process.cwd(), './__test__')

test('output can be captured', async (t) => {
  const res = await exec('echo hey')
  t.is(res.output.trim(), 'hey')
})

test('colors are captured', async (t) => {
  const res = await exec('node -e console.log(100)')
  t.is(res.output.trim(), formatWithOptions({ colors: true }, 100))
})

test('cwd option works', async (t) => {
  const res = await exec('node ./simple-cmd', { cwd })
  t.is(res.output.trim(), 'hello world')
})

test('timeout works', async (t) => {
  const res = await exec('node ./long-running-cmd', { timeout: 2, cwd })
  t.is(res.truncated, true)
})

test('errors are handled', async (t) => {
  await t.throwsAsync(() => exec('node ./doesnt-exist', { cwd }))
})
