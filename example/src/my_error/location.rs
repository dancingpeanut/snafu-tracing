use core::fmt;

#[derive(Copy, Clone)]
#[non_exhaustive]
pub struct Location {
    /// The file where the error was reported
    pub file: &'static str,
    /// The line where the error was reported
    pub line: u32,
    /// The column where the error was reported
    pub column: u32,
}

impl Location {
    /// Constructs a `Location` using the given information
    pub fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

impl Default for Location {
    #[track_caller]
    fn default() -> Self {
        let loc = core::panic::Location::caller();
        Self {
            file: loc.file(),
            line: loc.line(),
            column: loc.column(),
        }
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Location")
            .field("file", &self.file)
            .field("line", &self.line)
            .field("column", &self.column)
            .finish()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{file}:{line}:{column}",
            file = self.file,
            line = self.line,
            column = self.column,
        )
    }
}