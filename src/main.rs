use std::fmt::format;
use std::fs::File;
use std::io::Read;
use clap::builder::Str;
use clap::Parser;
use csv::StringRecord;

#[derive(Parser, Debug)]
#[clap(
    name = "c2s",
    author = "rinotc",
    version = "0.1",
    about = "csv to insert sql."
)]
struct Args {
    csv_file_path: String
}

fn main() {
    let args: Args = Args::parse();
    assert!(args.csv_file_path.contains(".csv"));
    println!("csv file path: {}", args.csv_file_path);
    let table_name = args.csv_file_path.replace(".csv", "");
    let mut file = File::open(args.csv_file_path).expect("file not found.");
    let mut csv_text = String::new();
    file.read_to_string(&mut csv_text)
        .expect("something went wrong read the file");

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_text.as_bytes());

    let mut column_names: Vec<String> = Vec::new();
    let mut row_num = 1;
    let mut column_count = 0;
    for record in reader.records() {
        let record: StringRecord = record.expect("something happened.");
        if row_num == 1 {
            column_count = record.len();
            for value in record.iter() {
                column_names.push(value.to_string());
            }
        } else {
            let mut sql = format!("INSERT INTO {} (", table_name);
            for c in &column_names {
                let s = format!(" {},", c);
                sql += s.as_str();
            }
            println!("{}", sql);
        }
        row_num += 1;
    }
}
