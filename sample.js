/* eslint-disable */

const { exec } = require('./lib')

exec('echo hey', {}, (err, res) => {
  console.log('err :>> ', err)
  console.log('res :>> ', res)
  console.log('res.output :>> ', res?.output)
})
