/* eslint-disable */

const { exec, execSync } = require('./lib')

console.log('[exec] before')
exec('node ./__test__/long-running-cmd', { timeout: 2 }, (err, res) => {
  console.log('err :>> ', err)
  console.log('res :>> ', res)
  console.log('res.output :>> ', res?.output)
})
console.log('[exec] after')

// console.log('[execSync] before')
// const res = execSync('node ./__test__/long-running-cmd', { timeout: 2 })
// console.log('[execSync] after: ', res)
