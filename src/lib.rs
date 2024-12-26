use std::collections::TryReserveError;
use std::env::current_dir;
use std::ffi::{OsStr, OsString};
use std::fmt::{Arguments, Display};
use std::path::{Path, PathBuf};
use std::cmp::Eq;

use serde::{Deserialize, Serialize};

/// [`PathBuf`] wrapper
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct PathBufD(PathBuf);

impl PathBufD {
    /// Creates a new [`PathBufD`]
    pub fn new() -> Self {
        Self(PathBuf::new())
    }

    /// Creates a new [`PathBufD`] with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(PathBuf::with_capacity(capacity))
    }

    /// Creates a new [`PathBufD`] in the current directory
    pub fn current() -> Self {
        Self(current_dir().unwrap_or(PathBuf::new()))
    }

    /// Coerces to a [`Path`] slice.
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    /// Gets `Vec<u8>` representation of the inner string.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_owned()
    }

    /// Extends self with path.
    pub fn push<P>(&mut self, path: P) -> ()
    where
        P: AsRef<Path>,
    {
        self.0.push(path)
    }

    /// Creates an owned [`PathBufD`] with path adjoined to self.
    pub fn join<P>(&self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self(self.0.join(path))
    }

    /// Truncates `self` to [`self.parent`]
    ///
    /// Returns `false` and does nothing if [`self.parent`] is [`None`].
    /// Otherwise, returns `true`.
    ///
    /// [`self.parent`]: Path::parent
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    /// Updates [`self.file_name`] to `file_name`
    ///
    /// [`self.file_name`]: Path::file_name
    pub fn set_file_name<S>(&mut self, file_name: S)
    where
        S: AsRef<OsStr>,
    {
        self.0.set_file_name(file_name);
    }

    /// Updates [`self.extension`] to `Some(extension)` or to `None` if `extension` is empty.
    ///
    /// [`self.extension`]: Path::extension
    pub fn set_extension<S>(&mut self, extension: S)
    where
        S: AsRef<OsStr>,
    {
        self.0.set_extension(extension);
    }

    /// Yields a mutable reference to the underlying [`OsString`] instance.
    pub fn as_mut_os_string(&mut self) -> &mut OsString {
        self.0.as_mut_os_string()
    }

    /// Consumes the [`PathBufD`], yielding its internal [`OsString`] storage.
    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }

    /// Converts this [`PathBufD`] into a [boxed](Box) [`Path`].
    pub fn into_boxed_path(self) -> Box<Path> {
        self.0.into_boxed_path()
    }

    /// Invokes [`capacity`] on the underlying instance of [`OsString`].
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Invokes [`clear`] on the underlying instance of [`OsString`].
    ///
    /// [`clear`]: OsString::clear
    pub fn clear(&mut self) -> () {
        self.0.clear()
    }

    /// Invokes [`reserve`] on the underlying instance of [`OsString`].
    ///
    /// [`reserve`]: OsString::reserve
    pub fn reserve(&mut self, additional: usize) -> () {
        self.0.reserve(additional)
    }

    /// Invokes [`try_reserve`] on the underlying instance of [`OsString`].
    ///
    /// [`try_reserve`]: OsString::try_reserve
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve(additional)
    }

    /// Invokes [`reserve_exact`] on the underlying instance of [`OsString`].
    ///
    /// [`reserve_exact`]: OsString::reserve_exact
    pub fn reserve_exact(&mut self, additional: usize) -> () {
        self.0.reserve_exact(additional)
    }

    /// Invokes [`try_reserve_exact`] on the underlying instance of [`OsString`].
    ///
    /// [`try_reserve_exact`]: OsString::try_reserve_exact
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve_exact(additional)
    }

    /// Invokes [`shrink_to_fit`] on the underlying instance of [`OsString`].
    ///
    /// [`shrink_to_fit`]: OsString::shrink_to_fit
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    /// Invokes [`shrink_to`] on the underlying instance of [`OsString`].
    ///
    /// [`shrink_to`]: OsString::shrink_to
    pub fn shrink_to(&mut self, min_capacity: usize) -> () {
        self.0.shrink_to(min_capacity)
    }

    /// Creates an owned [`PathBufD`] with all paths from `paths` adjoined to self.
    pub fn extend<P>(self, paths: &[P]) -> Self
    where
        P: AsRef<Path>,
    {
        let mut buf = self;

        for path in paths {
            buf.push(path)
        }

        buf
    }
}

impl Default for PathBufD {
    fn default() -> Self {
        Self(PathBuf::default())
    }
}

impl Display for PathBufD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_str().unwrap_or(""))
    }
}

impl AsRef<Path> for PathBufD {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl Into<PathBufD> for PathBuf {
    fn into(self) -> PathBufD {
        PathBufD(self)
    }
}

impl From<PathBufD> for PathBuf {
    fn from(value: PathBufD) -> Self {
        value.0
    }
}

// macro
/// Format [`Arguments`] into a [`PathBufD`]
pub fn pathbufd_fmt(args: Arguments) -> PathBufD {
    let string = if let Some(s) = args.as_str() {
        s
    } else {
        &args.to_string()
    };

    let mut pathbufd = PathBufD::new();
    for split in string.split("/") {
        if split.is_empty() {
            pathbufd.push("/");
            continue;
        }

        pathbufd.push(split);
    }

    return pathbufd;
}

#[macro_export]
macro_rules! pathd {
    ($($arg:tt)*) => {
        pathbufd::pathbufd_fmt(std::format_args!($($arg)*)).to_string()
    }
}
