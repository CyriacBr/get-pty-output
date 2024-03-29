/* eslint-disable */
const { exec, execSync } = require('./lib')

const onData = (_, line) => console.log('line :>> ', line);

console.log('[exec] before')
exec('cmd /c node ./__test__/long-running-cmd', {}, onData, (err, res) => {
  console.log('err :>> ', err)
  console.log('res :>> ', res)
  console.log('res.output :>> ', res?.output || '')
})
console.log('[exec] after')

// console.log('[execSync] before')
// const res = execSync('node ./__test__/long-running-cmd', { timeout: 2 })
// console.log('[execSync] after: ', res)

// console.log(`a \x1B[2K b`);
// console.log(`a \x1B[1J b`);
// console.log(`a \x1B[2J b`);
// console.log(`\x1B[1G\x1B[?25h\x1B[32m✔\x1B[39m Loading unicors\n`);

// console.log(`a \x1B[2K b`.replace(/\x1B\[2K/g, ""));
