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
    table_name: Option<String>,
}

fn main() {
    let args: Args = Args::parse();
    assert!(args.csv_file_path.contains(".csv"));
    let table_name = args.table_name.unwrap_or(args.csv_file_path.replace(".csv", ""));
    let mut file = File::open(args.csv_file_path).expect("file not found.");
    let mut csv_text = String::new();
    file.read_to_string(&mut csv_text)
        .expect("something went wrong read the file");

    let sqls = csv_2_insert_sql(csv_text, table_name);
    sqls.iter().for_each(|sql| {
        println!("{}", sql)
    });
}

fn csv_2_insert_sql(csv_text: String, table_name: String) -> Vec<String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_text.as_bytes());

    let mut column_names: Vec<String> = Vec::new();
    let mut row_num = 1;
    let mut insert_sql_list = Vec::new();
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
                let s = if is_num_str(value) || is_null(value) {
                    format!(" {},", value)
                } else { format!(" '{}',", value) };

                sql += s.as_str();
            }
            sql.pop();
            sql += " );";
            insert_sql_list.push(sql);
        }
        row_num += 1;
    }
    insert_sql_list.to_vec()
}

fn is_num_str(s: &str) -> bool {
    let n_int = s.parse::<isize>();
    let n_float = s.parse::<f64>();
    n_int.is_ok() || n_float.is_ok()
}

fn is_null(s: &str) -> bool {
    s == "null" || s == "NULL"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_num_str_i32_max_str_should_be_true() {
        assert_eq!(is_num_str(i32::MAX.to_string().as_str()), true);
    }

    #[test]
    fn is_num_str_i32_min_str_should_be_true() {
        assert_eq!(is_num_str(i32::MIN.to_string().as_str()), true);
    }

    #[test]
    fn is_num_str_float_max_str_should_be_true() {
        assert_eq!(is_num_str(1.7976931348623157e308.to_string().as_str()), true);
    }

    #[test]
    fn is_num_str_float_min_str_should_be_true() {
        assert_eq!(is_num_str((-1.7976931348623157e308).to_string().as_str()), true);
    }

    #[test]
    fn is_num_str_str_should_be_false() {
        assert_eq!(is_num_str("abcdefghijklmnopqrstuvwxyz"), false);
        assert_eq!(is_num_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), false);
        assert_eq!(is_num_str("あ井上お"), false);
    }

    #[test]
    fn is_null_test() {
        assert_eq!(is_null("null"), true);
        assert_eq!(is_null("nil"), false);
        assert_eq!(is_null("NULL"), true)
    }

    #[test]
    fn csv_2_insert_sql_test() {
        let csv_text = "\
user_id,email,user_name,height,weight,birthday
1,a@example.com,太郎,172.5,null,2022-05-05
2,b@example.com,二郎,182.3,92.03,null";
        let table_name = "users";
        let actual = csv_2_insert_sql(csv_text.to_string(), table_name.to_string());

        let expect1 = "INSERT INTO users ( user_id, email, user_name, height, weight, birthday ) VALUES ( 1, 'a@example.com', '太郎', 172.5, null, '2022-05-05' );";
        let expect2 = "INSERT INTO users ( user_id, email, user_name, height, weight, birthday ) VALUES ( 2, 'b@example.com', '二郎', 182.3, 92.03, null );";

        assert_eq!(actual[0], expect1.to_string());
        assert_eq!(actual[1], expect2.to_string());
    }
}