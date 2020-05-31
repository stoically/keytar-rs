//! [keytar](https://github.com/atom/node-keytar) bindings for Rust
//!
//! A native Node module to get, add, replace, and delete passwords in system's keychain.
//! On macOS the passwords are managed by the Keychain, on Linux they are managed by
//! the Secret Service API/libsecret, and on Windows they are managed by Credential Vault.
//!
//! ```
//! let service = "service".to_owned();
//! let account = "account".to_owned();
//! let password = "password".to_owned();
//!
//! # {
//! # let service = service.clone();
//! # let account = account.clone();
//! # let password = password.clone();
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

#[cxx::bridge]
pub mod ffi {
    /// Workaround until `cxx` supports `Option`s
    /// https://github.com/dtolnay/cxx/issues/87
    pub struct Password {
        success: bool,
        password: String,
    }

    extern "C" {
        include!("src/lib.h");

        /// Save the password for the service and account to the keychain. Adds a new entry if
        /// necessary, or updates an existing entry if one exists.
        ///
        /// Equivalent to [`setPassword`](https://github.com/atom/node-keytar#setpasswordservice-account-password)
        pub fn set_password(service: String, account: String, password: String) -> Result<()>;

        /// Get the stored password for the service and account.
        ///
        /// Equivalent to [`getPassword`](https://github.com/atom/node-keytar#getpasswordservice-account)
        pub fn get_password(service: String, account: String) -> Result<Password>;

        /// Delete the stored password for the service and account.
        ///
        /// Equivalent to [`deletePassword`](https://github.com/atom/node-keytar#deletepasswordservice-account)
        pub fn delete_password(service: String, account: String) -> Result<bool>;

        /// Find a password for the service in the keychain.
        ///
        /// Equivalent to [`findPassword`](https://github.com/atom/node-keytar#findpasswordservice)
        pub fn find_password(service: String) -> Result<Password>;
    }
}

pub use ffi::*;
