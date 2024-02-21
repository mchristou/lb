const http = require("http");

const portArg = process.argv[2];
if (!portArg) {
  console.error("Error: Port number is required.");
  console.log("Usage: node server.js <port>");
  process.exit(1); // Exit the script with an error code
}

const PORT = parseInt(portArg, 10);
const ADDR = `http://localhost:${PORT}/`;

const server = http.createServer((req, res) => {
  res.writeHead(200, { "Content-Type": "text/plain" });
  res.end(`Hello from ${ADDR}\n`);
});

server.listen(PORT, () => {
  console.log(`Server running at ${ADDR}`);
});
