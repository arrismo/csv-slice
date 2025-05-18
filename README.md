# csv-slice
Extract rows or columns from CSV files without loading the entire file


## Installation
Clone the repository and build with Cargo:
```sh
cargo build --release
```

## CLI Usage
```
csv-slice rows --input <csv_file> --start <start> --end <end>
csv-slice columns --input <csv_file> --columns <col1> <col2> ...
```

Example:
```rust
use csv_slice::{extract_rows, extract_columns};

let rows = extract_rows("data.csv", 0, 10)?;
let cols = extract_columns("data.csv", &["Name", "Email"])?;
```

## Examples
Run example programs:
```sh
cargo run --example rows -- data.csv 0 10
cargo run --example columns -- data.csv Name Email
```

