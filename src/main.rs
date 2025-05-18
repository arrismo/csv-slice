// create cli interface 

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "csv-slice")]
#[command(about = "Extract rows or columns from CSV files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Rows {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        start: usize,
        #[arg(short, long)]
        end: usize,
    },
    Columns {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        columns: Vec<String>,
    },
}

use csv_slice::CsvSliceError;

fn main() -> Result<(), CsvSliceError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Rows { input, start, end } => {
            let rows = csv_slice::extract_rows(input, *start, *end)?;
            for row in rows {
                println!("{}", row.iter().collect::<Vec<_>>().join(","));
            }
        }
        Commands::Columns { input, columns } => {
            let cols = csv_slice::extract_columns(input, &columns.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
            for row in cols {
                println!("{}", row.join(","));
            }
        }
    }
    Ok(())
}
