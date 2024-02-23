# Rust Load Balancer

This is a simple Rust load balancer implementation using Tokio for asynchronous IO and concurrency.
It utilizes a round-robin algorithm to distribute incoming requests among multiple backend servers.

## Usage

```
./lb <backend_andresses>
```

## Testing

To simulate backend servers, initiate multiple servers on localhost using different ports.

```
node server/server.js <port>
```

Start the load balancer:

```
./lb 127.0.0.1::8081 127.0.0.1::8082
```

Once the load balancer is operational, you can issue HTTP requests to it.
It will distribute incoming requests among the available backend servers.

```
curl http://localhost:8080
```
