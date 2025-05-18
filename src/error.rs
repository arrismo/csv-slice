// Import the thiserror crate which provides macros for easier error handling in Rust
use thiserror::Error;

/// CsvSliceError is the main error type for the csv-slice library.
/// 
/// This enum represents all possible errors that can occur when working with the csv-slice library.
/// It uses the thiserror crate to automatically implement the Error trait and provide
/// formatted error messages.
///
/// # Error Types
/// - `Csv`: Represents errors from the csv crate when parsing or processing CSV files
/// - `Io`: Represents standard I/O errors that may occur when reading files
/// - `ColumnNotFound`: A custom error that occurs when a requested column name doesn't exist in the CSV
#[derive(Error, Debug)]
pub enum CsvSliceError {
    /// Wraps errors from the csv crate
    /// The #[from] attribute automatically implements From<csv::Error> for CsvSliceError
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    /// Wraps standard I/O errors
    /// The #[from] attribute automatically implements From<std::io::Error> for CsvSliceError
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Custom error for when a requested column name is not found in the CSV headers
    /// Contains the name of the column that was not found
    #[error("Column not found: {0}")]
    ColumnNotFound(String)
}