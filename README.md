# tcp-splitter

[![Build status][build-badge]][build-url]
[![Version][version-badge]][version-url]
[![MIT licensed][mit-badge]][mit-url]

[build-badge]: https://travis-ci.com/benceszigeti/tcp-splitter.svg?branch=master
[build-url]: https://travis-ci.com/benceszigeti/tcp-splitter
[version-badge]: https://img.shields.io/github/release/benceszigeti/tcp-splitter.svg
[version-url]: https://github.com/benceszigeti/tcp-splitter/releases/latest
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

TCP proxy server with the ability to copy clients upstream to sniffer servers.

## Documentation

Stay tuned!

## Supported Rust versions

**This project requires Rust nightly and does not provide API stability
guarantees.**

**You are living on the edge here.**

## Usage

```
$ ./tcp-splitter --help
tcp-splitter v0.1.0-rc0
Bence SZIGETI <bence.szigeti@gohyda.com>
TCP proxy server with the ability to copy clients upstream to sniffer servers.

USAGE:
    tcp-splitter [OPTIONS] --listen <listen address> --proxied <proxied server address>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --listen <listen address>                Listen address
    -p, --proxied <proxied server address>       Proxied channels: rx/tx
    -s, --sniffer <sniffer server address>...    Proxied channels: rx
```

### Usage example

`tcp-splitter` server:
```
$ LOG=tcp_splitter,tcp_splitter_cli=info cargo +nightly run --release -- \
    --listen 127.0.0.1:1202 \
    --proxied 127.0.0.1:5000 \
    --sniffer 127.0.0.1:6000 \
    --sniffer 127.0.0.1:7000
```

Proxied server:
```
$ iperf -s -p 5000 -b 800Mbits/sec
```

Sniffer server #1:
```
$ iperf -s -p 6000 -b 1Gbytes/sec
```

Sniffer server #2:
```
$ iperf -s -p 7000 -b 500Mbits/sec
```

Client:
```
$ iperf -c 127.0.0.1 -p 1202 -n 250Mbytes -P 4
```

## License

This project is licensed under the [MIT license](LICENSE).
