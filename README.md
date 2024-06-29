# unitycatalog-rs

[![CI](https://github.com/unitycatalog/unitycatalog-rs/workflows/CI/badge.svg)](https://github.com/unitycatalog/unitycatalog-rs/actions) ![crates.io](https://img.shields.io/crates/v/unitycatalog.svg?label=unitycatalog) ![crates.io](https://img.shields.io/crates/v/unitycatalog-client.svg?label=unitycatalog-client) ![crates.io](https://img.shields.io/crates/v/unitycatalog-sys.svg?label=unitycatalog-sys)

**Unity Catalog Rust (UC-R)**

A native Rust implementation of the [Unity Catalog](https://github.com/unitycatalog/unitycatalog): an Open and Multimodal Catalog for Data & AI. 

## Prerequisites

- Install Rust, e.g. as described [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## [WIP] Quick Start

Use two terminal windows: one to run the Unity Catalog Rust server (UC-R) another to explore the content of the UC-R server using varios client tools.

### Run the server

In the first terminal window, start the UC server (from the repository root directory)

```
cargo run
```

### [TBD] Query with DuckDB

- [Install](https://duckdb.org/docs/installation/) DuckDB
- Start DuckDB (run `duckdb` in the terminal)
- Run the following commands in the DuckDB shell to install the required extensions
```
install uc_catalog from core_nightly;
load uc_catalog;
install delta;
load delta;
```

- Attach the unity catalog to DuckDB
```
ATTACH 'unity' AS unity (TYPE UC_CATALOG);
```

- Run the queries, for example
```
SHOW ALL TABLES;
SELECT * from unity.default.numbers;
```

## APIs and Compatibility

- Open API specification: The Unity Catalog Rest API is documented [here](https://github.com/unitycatalog/unitycatalog/tree/main/api).
- Compatibility and stability: The APIs are currently evolving and should not be assumed to be stable.

## Compiling and Testing

```
cargo test
```

## Why Rust?

Rust was chosen as a language for the alternative implementation for a few reasons:

- **Performance**: Rust compiles to native bytecode, enabling it to be used in performance-critical and constrained environments.
- **FFI**: As Rust uses the C ABI, bindings can be exposed to any language, like C, Go, etc.
- **Python & Rust**: Python and Rust are a strong pairing, especially in the AI ecosystem ([tokenizers](https://github.com/huggingface/tokenizers), [pydantic](https://github.com/pydantic/pydantic-core), etc).
- **Ecosystem**: Rust has a rich ecosystem of libraries, especially related to big data processing ([delta-rs](https://github.com/delta-io/delta-rs), [datafusion](https://github.com/apache/datafusion), etc).
- **Safety**: Rust's strong type system and ownership model make it difficult to write code that is unsafe or has undefined behavior. When dealing with data assets, especially when it comes to access control, this helps us ensure that we are not introducing security vulnerabilities.
- **We want to write it!**: Most importantly, we like writing Rust!

## Getting Involved

We encourage you to reach out and share your feedback or ideas by [raising an issue](issues/new). 

See [CONTRIBUTING.md](CONTRIBUTING.md) if you are looking to contribute.