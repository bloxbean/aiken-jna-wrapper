# aiken-jna-wrapper
[![Build](https://github.com/bloxbean/aiken-jna-wrapper/actions/workflows/build.yml/badge.svg)](https://github.com/bloxbean/aiken-jna-wrapper/actions/workflows/build.yml)
[![Release](https://github.com/bloxbean/aiken-jna-wrapper/actions/workflows/release.yml/badge.svg)](https://github.com/bloxbean/aiken-jna-wrapper/actions/workflows/release.yml)

A thin native wrapper that exposes [Aiken](https://aiken-lang.org/)'s `uplc`
(Untyped Plutus Core) engine as a plain C-ABI shared library, so the JVM can
call it through [JNA](https://github.com/java-native-access/jna).

It lets Java/Kotlin applications work with Cardano Plutus scripts — primarily
**phase-2 evaluation**, i.e. computing the execution-unit (CPU/memory) budgets a
transaction's scripts consume — by reusing Aiken's Rust Plutus VM (`uplc`)
instead of reimplementing it. The prebuilt, per-platform binaries it releases are
consumed by the [aiken-java-binding](https://github.com/bloxbean/aiken-java-binding)
project.

## Exposed functions

- **Phase-2 transaction ExUnits evaluation** (`eval_phase_two`) — evaluates the
  scripts in a transaction and returns the redeemers with their computed
  execution-unit budgets.
- **Apply parameters to a Plutus script** (`apply_params_to_plutus_script`) —
  applies CBOR-encoded parameters to a parameterized compiled script.

Both are exported as C functions (`extern "C"`) taking/returning hex- or
CBOR-encoded strings, with results serialized as JSON. Strings returned across
the boundary are owned by the caller and must be released once, after reading,
via `dropCharPointer`.

## Supported operating systems / architectures

- Apple macOS (Intel `x86_64` and Apple Silicon `aarch64`)
- Linux (`x86_64` and `aarch64`) — built on Ubuntu 24.04, compatible with other
  reasonably recent distributions
- Windows (`x86_64` and `aarch64`)

For another platform, please open a request [here](https://github.com/bloxbean/aiken-jna-wrapper/issues).

## Build

```shell
git clone https://github.com/bloxbean/aiken-jna-wrapper.git
```

```shell
cargo test
cargo build --release
```

The build produces a dynamic library (`.so` / `.dylib` / `.dll`) under
`target/release/`.
