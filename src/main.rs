use std::fs::File;
use std::io::Read;
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
    csv_file_path: String,
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
    for record in reader.records() {
        let record: StringRecord = record.expect("something happened.");
        if row_num == 1 {
            for value in record.iter() {
                column_names.push(value.to_string());
            }
        } else {
            let mut sql = format!("INSERT INTO {} (", table_name);
            for c in &column_names {
                let s = format!(" {},", c);
                sql += s.as_str();
            }
            sql.pop();
            sql += " ) VALUES (";
            for value in record.iter() {
                let s = if is_num_str(value) {
                    format!(" {},", value)
                } else { format!(" '{}',", value) };

                sql += s.as_str();
            }
            sql.pop();
            sql += " );";
            println!("{}", sql);
        }
        row_num += 1;
    }
}

fn is_num_str(s: &str) -> bool {
    let n_int = s.parse::<isize>();
    let n_float = s.parse::<f64>();
    n_int.is_ok() || n_float.is_ok()
}
