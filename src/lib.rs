// src/lib.rs
use csv::StringRecord;
use std::fs::File;
use std::io::{self, BufReader};

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