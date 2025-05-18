// src/lib.rs
// This library provides utilities for extracting specific rows and columns from CSV files.
// It offers two main functions:
// 1. extract_rows: Extracts a range of rows from a CSV file
// 2. extract_columns: Extracts specific columns from a CSV file by column name

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    /// Test for the extract_rows function
    /// Creates a temporary CSV file with sample data and verifies that
    /// the correct rows are extracted based on the provided range.
    #[test]
    fn test_extract_rows() {
        // Create sample CSV data with header and 3 data rows
        let csv_data = "Name,Age\nAlice,30\nBob,25\nCharlie,40\n";
        
        // Create a temporary file that will be automatically deleted when the test completes
        let mut file = NamedTempFile::new().unwrap();
        
        // Write the sample CSV data to the temporary file
        write!(file, "{}", csv_data).unwrap();
        
        // Get the path to the temporary file
        let path = file.path();
        
        // Extract rows 0 and 1 (Alice and Bob) from the CSV file
        let rows = super::extract_rows(path, 0, 2).unwrap();
        
        // Verify that exactly 2 rows were extracted
        assert_eq!(rows.len(), 2);
        
        // Verify that the first row contains "Alice" in the first column
        assert_eq!(rows[0].get(0), Some("Alice"));
        
        // Verify that the second row contains "Bob" in the first column
        assert_eq!(rows[1].get(0), Some("Bob"));
        // The temporary file is automatically deleted when it goes out of scope
    }

    /// Test for the extract_columns function
    /// Creates a temporary CSV file and verifies that the correct column
    /// data is extracted when a valid column name is provided.
    #[test]
    fn test_extract_columns() {
        // Create sample CSV data with header and 3 data rows
        let csv_data = "Name,Age\nAlice,30\nBob,25\nCharlie,40\n";
        
        // Create a temporary file that will be automatically deleted
        let mut file = NamedTempFile::new().unwrap();
        
        // Write the sample CSV data to the temporary file
        write!(file, "{}", csv_data).unwrap();
        
        // Get the path to the temporary file
        let path = file.path();
        
        // Extract the "Name" column from the CSV file
        let columns = super::extract_columns(path, &["Name"]).unwrap();
        
        // Verify that 3 rows of data were extracted
        assert_eq!(columns.len(), 3);
        
        // Verify the values in the extracted column
        assert_eq!(columns[0][0], "Alice");
        assert_eq!(columns[1][0], "Bob");
        assert_eq!(columns[2][0], "Charlie");
    }

    /// Test for error handling in extract_columns
    /// Verifies that an error is returned when attempting to extract
    /// a column that doesn't exist in the CSV file.
    #[test]
    fn test_extract_columns_not_found() {
        // Create sample CSV data with header and 1 data row
        let csv_data = "Name,Age\nAlice,30\n";
        
        // Create a temporary file
        let mut file = NamedTempFile::new().unwrap();
        
        // Write the sample CSV data to the temporary file
        write!(file, "{}", csv_data).unwrap();
        
        // Get the path to the temporary file
        let path = file.path();
        
        // Attempt to extract a column that doesn't exist ("Email")
        let result = super::extract_columns(path, &["Email"]);
        
        // Verify that an error is returned
        assert!(result.is_err());
    }
}
// END TESTS
// Import required dependencies
use csv::StringRecord;  // For handling CSV records
use std::fs::File;      // For file operations
use std::io::BufReader; // For buffered reading from files
mod error;              // Import the error module
pub use crate::error::CsvSliceError; // Re-export the CsvSliceError type

/// Extracts a range of rows from a CSV file.
///
/// # Parameters
/// * `path` - Path to the CSV file. Can be any type that can be converted to a Path.
/// * `start` - The index of the first row to extract (0-based, excluding header).
/// * `end` - The index after the last row to extract (exclusive).
///
/// # Returns
/// * `Result<Vec<StringRecord>, CsvSliceError>` - A vector of StringRecords on success,
///   or a CsvSliceError on failure.
///
/// # Example
/// ```
/// use csv_slice::extract_rows;
/// use std::io::Write;
/// use std::fs::File;
/// 
/// // Create a temporary CSV file for the example
/// let temp_dir = tempfile::tempdir().unwrap();
/// let file_path = temp_dir.path().join("sample.csv");
/// let mut file = File::create(&file_path).unwrap();
/// writeln!(file, "Name,Age\nAlice,30\nBob,25\nCharlie,40").unwrap();
/// 
/// // Now extract the first 2 rows
/// let rows = extract_rows(&file_path, 0, 2).unwrap();
/// assert_eq!(rows.len(), 2);
/// assert_eq!(rows[0].get(0), Some("Alice"));
/// ```
pub fn extract_rows<P: AsRef<std::path::Path>>(
    path: P,
    start: usize,
    end: usize,
) -> Result<Vec<StringRecord>, CsvSliceError> {
    // Open the file at the specified path
    let file = File::open(path)?;
    
    // Create a CSV reader with buffered IO for better performance
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    
    // Initialize an empty vector to store the results
    let mut result = Vec::new();

    // Iterate through all records in the CSV file
    for (i, record) in rdr.records().enumerate() {
        // Parse the record, propagating any errors
        let record = record?;
        
        // If the current index is within our desired range, add it to the results
        if i >= start && i < end {
            result.push(record);
        }
        
        // If we've reached the end of our desired range, stop processing
        if i >= end {
            break;
        }
    }
    
    // Return the collected results
    Ok(result)
}

/// Extracts specific columns from a CSV file by column name.
///
/// # Parameters
/// * `path` - Path to the CSV file. Can be any type that can be converted to a Path.
/// * `columns` - Array of column names to extract.
///
/// # Returns
/// * `Result<Vec<Vec<String>>, CsvSliceError>` - A vector of vectors containing the
///   extracted column data on success, or a CsvSliceError on failure.
///
/// # Example
/// ```
/// use csv_slice::extract_columns;
/// use std::io::Write;
/// use std::fs::File;
/// 
/// // Create a temporary CSV file for the example
/// let temp_dir = tempfile::tempdir().unwrap();
/// let file_path = temp_dir.path().join("sample.csv");
/// let mut file = File::create(&file_path).unwrap();
/// writeln!(file, "Name,Email,Age\nAlice,alice@example.com,30\nBob,bob@example.com,25").unwrap();
/// 
/// // Now extract the Name and Email columns
/// let data = extract_columns(&file_path, &["Name", "Email"]).unwrap();
/// assert_eq!(data.len(), 2); // Two rows of data
/// assert_eq!(data[0][0], "Alice"); // First row, Name column
/// assert_eq!(data[0][1], "alice@example.com"); // First row, Email column
/// ```
pub fn extract_columns<P: AsRef<std::path::Path>>(
    path: P,
    columns: &[&str],
) -> Result<Vec<Vec<String>>, CsvSliceError> {
    // Open the file at the specified path
    let file = File::open(path)?;
    
    // Create a CSV reader with buffered IO for better performance
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    
    // Get the headers from the CSV file and clone them for later use
    let headers = rdr.headers()?.clone();

    // Find the indices of the requested columns in the header row
    let indices: Vec<_> = columns
        .iter()
        .map(|&col| headers.iter().position(|h| h == col)
            // If a column is not found, return a ColumnNotFound error
            .ok_or_else(|| CsvSliceError::ColumnNotFound(col.to_string())))
        .collect::<Result<_, _>>()?;

    // Initialize an empty vector to store the results
    let mut result = Vec::new();
    
    // Process each record in the CSV file
    for record in rdr.records() {
        // Parse the record, propagating any errors
        let record = record?;
        
        // Extract the values from the requested columns for this record
        let row: Vec<String> = indices.iter()
            .map(|&i| record.get(i).unwrap_or("").to_string())
            .collect();
            
        // Add the extracted values to the result
        result.push(row);
    }
    
    // Return the collected results
    Ok(result)
}