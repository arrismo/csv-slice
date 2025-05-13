use thiserror::Error;

// thiserror crate allows me to define custom error types in Rust
// enum is short for enumerate, enum can hold data and defines types
// pub is short for public, pub makes them accessible from outside our file


#[derive(Error, Debug)]
pub enum CsvSliceError {
    #[error("CSV error: {0}")] // This line is a custom error message for CsvSliceError, it will print the error message with the error itself
    Csv(#[from] csv::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Column not found: {0}")]
    ColumnNotFound(String)
}