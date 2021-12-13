/* eslint-disable */

// import * as Path from 'path'
// import { formatWithOptions } from 'util'

import test from 'tape'

import { exec } from '../index'

// const cwd = Path.join(process.cwd(), './__test__')

test('binding exists', async (t) => {
  t.ok(exec)
})

// test('output can be captured', async (t) => {
//   const res = await exec('echo hey')
//   t.ok(res.output.includes('hey'))
// })

// test.skip('colors are captured', async (t) => {
//   const res = await exec('node -e console.log(100)')
//   t.is(res.output.trim(), formatWithOptions({ colors: true }, 100))
// })

// test('cwd option works', async (t) => {
//   const res = await exec('node ./simple-cmd', { cwd })
//   t.true(res.output.includes('hello world'))
// })

// test('timeout works', async (t) => {
//   const res = await exec('node ./long-running-cmd', { timeout: 2, cwd })
//   t.is(res.truncated, true)
// })

// test('errors are handled', async (t) => {
//   try {
//     await exec('node ./doesnt-exist', { cwd })
//   } catch (error) {
//     t.ok(error)
//   }
// })
