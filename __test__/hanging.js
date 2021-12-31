/* eslint-disable no-console */
const http = require('http')

const requestListener = function (req, res) {
  res.writeHead(200)
  res.end('Hello, World!')
}

const server = http.createServer(requestListener)
server.listen(8080)
console.log('Server listening at http://localhost:8080')
