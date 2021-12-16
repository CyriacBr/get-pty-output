const { exec, execSync } = require('./lib')

function execPromise(cmd, opts) {
  return new Promise((resolve, reject) => {
    exec(cmd, opts || {}, (err, res) => {
      if (err) {
        reject(err)
      } else {
        resolve(res)
      }
    })
  })
}

module.exports.exec = execPromise
module.exports.execSync = function (cmd, opts) {
  return execSync(cmd, opts || {})
}
