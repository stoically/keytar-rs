# keytar-rs

[![crates.io page](https://img.shields.io/crates/v/keytar.svg)](https://crates.io/crates/keytar)
[![docs.rs page](https://docs.rs/keytar/badge.svg)](https://docs.rs/keytar/)
![build](https://github.com/stoically/keytar-rs/workflows/build/badge.svg)
![license: MIT](https://img.shields.io/crates/l/keytar.svg)

[keytar](https://github.com/atom/node-keytar) bindings for Rust

**Note:** You might want to consider using the [keyring](https://crates.io/crates/keyring) crate instead, which is fully written in Rust.

A native Node module to get, add, replace, and delete passwords in system's keychain. On macOS the passwords are managed by the Keychain, on Linux they are managed by the Secret Service API/libsecret, and on Windows they are managed by Credential Vault.

```rust
let service = "service";
let account = "account";
let password = "password";

keytar::set_password(service, account, password).unwrap();
```

## Linux Requirement

Currently this library uses `libsecret`. Depending on your distribution,
you will need to install the appropriate package, e.g.:

- Debian/Ubuntu: `sudo apt-get install libsecret-1-dev`
- Red Hat-based: `sudo yum install libsecret-devel`
- Arch Linux: `sudo pacman -S libsecret`
