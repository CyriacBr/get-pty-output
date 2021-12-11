const { exec } = require('./lib')

/**
 * Execute a command and return it's output
 * @param {string} cmd
 * @param {import('./index').Options} opts
 * @returns {Promise<import('./index').ReturnType>}
 */
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
