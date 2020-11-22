#[cxx::bridge]
pub mod ffi {
    /// Workaround until `cxx` supports `Option`s
    /// https://github.com/dtolnay/cxx/issues/87
    pub struct Password {
        success: bool,
        password: String,
    }

    unsafe extern "C++" {
        include!("src/lib.h");

        /// Save the password for the service and account to the keychain. Adds a new entry if
        /// necessary, or updates an existing entry if one exists.
        ///
        /// Equivalent to [`setPassword`](https://github.com/atom/node-keytar#setpasswordservice-account-password)
        pub fn set_password(service: &str, account: &str, password: &str) -> Result<()>;

        /// Get the stored password for the service and account.
        ///
        /// Equivalent to [`getPassword`](https://github.com/atom/node-keytar#getpasswordservice-account)
        pub fn get_password(service: &str, account: &str) -> Result<Password>;

        /// Delete the stored password for the service and account.
        ///
        /// Equivalent to [`deletePassword`](https://github.com/atom/node-keytar#deletepasswordservice-account)
        pub fn delete_password(service: &str, account: &str) -> Result<bool>;

        /// Find a password for the service in the keychain.
        ///
        /// Equivalent to [`findPassword`](https://github.com/atom/node-keytar#findpasswordservice)
        pub fn find_password(service: &str) -> Result<Password>;
    }
}
