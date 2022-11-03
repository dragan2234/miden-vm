use crate::errors::LibraryError;

/// TODO: add docs
pub trait Library {
    type Module;

    /// Returns the root namespace of this library.
    fn root_ns(&self) -> &str;

    /// Returns the version number of this library.
    fn version(&self) -> &str;

    /// Returns the module located at the specified path.
    ///
    /// # Errors
    /// Returns an error if the modules for the specified path does not exist in this library.
    fn get_module(&self, module_path: &str) -> Result<&Self::Module, LibraryError>;
}
