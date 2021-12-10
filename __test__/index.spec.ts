import { format } from 'util'

import test from 'ava'

import { plus100, exec } from '../index'

function execPromise(cmd: string) {
  return new Promise((resolve, reject) => {
    exec(cmd, {}, (err, str) => {
      if (err) {
        reject(err)
      } else {
        resolve(str)
      }
    })
  })
}

test('sync function from native code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
})

test('output can be captured', async (t) => {
  const res = await execPromise('echo hey')
  t.is(res, 'hey')
})
