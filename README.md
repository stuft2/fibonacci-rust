# Fibonacci Web Server (Rust)

## Usage

Starting the server:

```shell
cargo run
```

## Benchmarking

This command:

- Uses 4 threads (-t4).
- Simulates 1000 concurrent connections (-c1000).
- Runs for 30 seconds (-d30s).

```shell
wrk -t4 -c1000 -d30s "http://localhost:8080/fibonacci?n=40"
```

#### Results with a 2021 MacBook Pro

- **Chip**: Apple M1 Pro
- **Memory**: 16 GB
- **macOS**: Sonoma 14.6.1

```shell
Running 30s test @ http://localhost:8080/fibonacci?n=40
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.96ms    8.22ms 129.85ms   83.25%
    Req/Sec    24.60k     2.59k   56.26k    94.17%
  2939450 requests in 30.09s, 437.88MB read
  Socket errors: connect 0, read 3346, write 0, timeout 0
Requests/sec:  97690.22
Transfer/sec:     14.55MB

```