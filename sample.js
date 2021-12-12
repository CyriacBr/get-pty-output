/* eslint-disable */

const { exec, test } = require('./lib')

exec('node -e "console.log(100)"', {}, (err, res) => {
  console.log('err :>> ', err);
  console.log('res :>> ', res.output)
})


const res = test();
console.log('test :>> ', res);