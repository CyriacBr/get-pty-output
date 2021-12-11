const ora = require('ora')

const spinner = ora('Loading unicorns').start()

console.log('\nUpdating stuff');

setTimeout(() => {
  // spinner.color = 'yellow'
  // spinner.text = 'Loading rainbows'
  spinner.succeed()
}, 1000)
