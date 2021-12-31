/* eslint-disable */
const AnsiToHTML = require('ansi-to-html')
const { exec, execSync } = require('./lib')

console.log('[exec] before')
exec('node ./__test__/stdout-update', {}, (err, res) => {
  console.log('err :>> ', err)
  console.log('res :>> ', res)
  console.log('res.output :>> ', res?.output || '')

  const converter = new AnsiToHTML();
  console.log(converter.toHtml(res?.output || ''));
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
