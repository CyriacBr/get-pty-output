const { exec } = require('./index')

exec('node ./idle-cmd.js', {}, (err, res) => {
  console.log('err :>> ', err)
  console.log('res :>> ', res)
  console.log(res.output);
})
