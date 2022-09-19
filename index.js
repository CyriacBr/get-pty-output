const { exec, execSync } = require("./lib");

function execPromise(cmd, opts) {
  const onData = (opts || {}).onData ? (err_, line) => opts.onData(line) : null;
  return new Promise((resolve, reject) => {
    exec(cmd, opts || {}, onData, (err, res) => {
      if (err) {
        reject(err);
      } else {
        resolve(res);
      }
    });
  });
}

module.exports.exec = execPromise;
module.exports.execSync = function (cmd, opts) {
  return execSync(cmd, opts || {});
};
