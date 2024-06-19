# unitycatalog-rs

[![CI](https://github.com/unitycatalog/unitycatalog-rs/workflows/CI/badge.svg)](https://github.com/unitycatalog/unitycatalog-rs/actions)

unitycatalog-rs serves an alternate implementation of the [Unity Catalog protocol](https://github.com/unitycatalog/unitycatalog/tree/main/api). The Unity Catalog protocol is a specification for a multi-modal catalog for data and AI assets.

## Why Rust?

Rust was chosen as a language for the second implementation for a few reasons:

- **Performance**: Rust compiles to native bytecode, enabling it to be used in performance-critical and constrained environments.
- **FFI**: As Rust uses the C ABI, bindings can be exposed to any language, like C, Go, Python, etc.
- **Ecosystem**: Rust has a rich ecosystem of libraries, especially related to big data processing ([delta-rs](https://github.com/delta-io/delta-rs), [datafusion](https://github.com/apache/datafusion), etc).
- **Safety**: Rust's strong type system and ownership model make it difficult to write code that is unsafe or has undefined behavior. When dealing with data assets, especially when it comes to access control, this helps us ensure that we are not introducing security vulnerabilities.
- **We want to write it!**: Most importantly, we like writing Rust!

## License

Licensed under Apache License, Version 2.0 ([LICENSE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
