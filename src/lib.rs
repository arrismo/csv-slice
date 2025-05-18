// src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_extract_rows() {
        let csv_data = "Name,Age\nAlice,30\nBob,25\nCharlie,40\n";
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", csv_data).unwrap();
        let path = file.path();
        let rows = super::extract_rows(path, 0, 2).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get(0), Some("Alice"));
        assert_eq!(rows[1].get(0), Some("Bob"));
        // file is deleted here
    }

    #[test]
    fn test_extract_columns() {
        let csv_data = "Name,Age\nAlice,30\nBob,25\nCharlie,40\n";
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", csv_data).unwrap();
        let path = file.path();
        let columns = super::extract_columns(path, &["Name"]).unwrap();
        assert_eq!(columns.len(), 3);
        assert_eq!(columns[0][0], "Alice");
        assert_eq!(columns[1][0], "Bob");
        assert_eq!(columns[2][0], "Charlie");
    }

    #[test]
    fn test_extract_columns_not_found() {
        let csv_data = "Name,Age\nAlice,30\n";
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", csv_data).unwrap();
        let path = file.path();
        let result = super::extract_columns(path, &["Email"]);
        assert!(result.is_err());
    }
}
// END TESTS
use csv::StringRecord;
use std::fs::File;
mod error;
pub use crate::error::CsvSliceError;
use std::io::BufReader;

pub fn extract_rows<P: AsRef<std::path::Path>>(
    path: P,
    start: usize,
    end: usize,
) -> Result<Vec<StringRecord>, CsvSliceError> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    let mut result = Vec::new();

    for (i, record) in rdr.records().enumerate() {
        let record = record?;
        if i >= start && i < end {
            result.push(record);
        }
        if i >= end {
            break;
        }
    }
    Ok(result)
}

pub fn extract_columns<P: AsRef<std::path::Path>>(
    path: P,
    columns: &[&str],
) -> Result<Vec<Vec<String>>, CsvSliceError> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    let headers = rdr.headers()?.clone();

    // Find indices of requested columns
    let indices: Vec<_> = columns
        .iter()
        .map(|&col| headers.iter().position(|h| h == col)
            .ok_or_else(|| CsvSliceError::ColumnNotFound(col.to_string())))
        .collect::<Result<_, _>>()?;

    let mut result = Vec::new();
    for record in rdr.records() {
        let record = record?;
        let row: Vec<String> = indices.iter().map(|&i| record.get(i).unwrap_or("").to_string()).collect();
        result.push(row);
    }
    Ok(result)
}