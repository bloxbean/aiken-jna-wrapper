# aiken-jna-wrapper

## Build

JNA wrapper for [Aiken](https://aiken-lang.org/) smart contract language. The binaries from this project are used in [aiken-java-binding](https://github.com/bloxbean/aiken-java-binding) project.

It exposes following functions
- Tx evaluation

## Supported Operating Systems / Archs
- Apple MacOS (Intel and Apple Silicon)
- Linux (x86_64) (Ubuntu 20.04 or compatible ...)
- Windows 64bits (x86_64)

For another platform, please create a PR / request [here](https://github.com/bloxbean/aiken-jna-wrapper/issues)

## Build

```shell
git clone https://github.com/bloxbean/aiken-jna-wrapper.git
```

```shell
cargo test
cargo build --release
```

