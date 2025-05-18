use csv_slice::extract_rows;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: cargo run --example rows -- <csv_file> <start> <end>");
        std::process::exit(1);
    }
    let path = &args[1];
    let start: usize = args[2].parse().unwrap();
    let end: usize = args[3].parse().unwrap();
    match extract_rows(path, start, end) {
        Ok(rows) => {
            for row in rows {
                println!("{}", row.iter().collect::<Vec<_>>().join(","));
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
