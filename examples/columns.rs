use csv_slice::extract_columns;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run --example columns -- <csv_file> <column1> [<column2> ...]");
        std::process::exit(1);
    }
    let path = &args[1];
    let columns: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
    match extract_columns(path, &columns) {
        Ok(cols) => {
            for row in cols {
                println!("{}", row.join(","));
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
