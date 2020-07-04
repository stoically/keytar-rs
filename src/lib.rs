//! [keytar](https://github.com/atom/node-keytar) bindings for Rust
//!
//! A native Node module to get, add, replace, and delete passwords in system's keychain.
//! On macOS the passwords are managed by the Keychain, on Linux they are managed by
//! the Secret Service API/libsecret, and on Windows they are managed by Credential Vault.
//!
//! ```
//! let service = "service";
//! let account = "account";
//! let password = "password";
//!
//! # {
//! # let service = service;
//! # let account = account;
//! # let password = password;
//! keytar::set_password(service, account, password).unwrap();
//! # }
//! #
//! # keytar::delete_password(service, account).unwrap();
//! ```
//!
//! ## Linux Requirement
//!
//! Currently this library uses `libsecret`. Depending on your distribution,
//! you will need to install the appropriate package, e.g.:
//!
//! - Debian/Ubuntu: `sudo apt-get install libsecret-1-dev`
//! - Red Hat-based: `sudo yum install libsecret-devel`
//! - Arch Linux: `sudo pacman -S libsecret`

pub use keytar_sys::ffi::*;
