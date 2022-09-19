/* eslint-disable no-console */
const ora = require("ora");

const spinner = ora("Loading unicorns").start();
setTimeout(() => {
  spinner.succeed();
}, 500);
