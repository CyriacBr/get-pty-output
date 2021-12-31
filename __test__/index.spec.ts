/* eslint-disable */
import * as Path from 'path'
import { formatWithOptions } from 'util'
import test from 'tape'
import { exec, execSync } from '../index'

const cwd = Path.join(process.cwd(), './__test__')

test('binding exists', async (t) => {
  t.ok(exec)
  t.ok(execSync)
})

/**
 * During CI, tty isn't available, and there's no way
 * around that for windows hosts
 */
if (!process.env.WINDOWS_CI) {
  test('output can be captured', async (t) => {
    const res = await exec('echo hey')
    t.ok(res.output.includes('hey'))
  })

  test('exec is non-blocking', (t) => {
    let i = 0
    exec('echo hey').then(() => {
      i++
      t.end()
    })
    t.equal(i, 0)
  })

  test('execSync is blocking', (t) => {
    const res = execSync('node -e "setTimeout(()=>{console.log(\'hey\')}, 2000)"')
    t.ok(res.output.includes('hey'))
    t.end()
  })

  test('colors are captured', async (t) => {
    const res = await exec('node -e console.log(100)')
    t.is(res.output.trim(), formatWithOptions({ colors: true }, 100))
  })

  test('cwd option works', async (t) => {
    const res = await exec('node ./simple-cmd', { cwd })
    t.true(res.output.includes('hello world'))
  })

  test('timeout works', async (t) => {
    const res = await exec('node ./long-running-cmd', { timeout: 2, cwd })
    t.is(res.truncated, true)
  })

  test('timeout works even for hanging processs', async (t) => {
    const res = await exec('node ./hanging', { timeout: 2, cwd })
    t.is(res.truncated, true)
  })

  test('errors are handled', async (t) => {
    try {
      await exec('node ./doesnt-exist', { cwd })
    } catch (error) {
      t.ok(error)
    }
  })
}
