# c2s

CSVファイルから、SQLのINSERT文をgenerateするツール

## How to install

### install via cargo

```shell
cargo install c2s
```

## How to use

以下のようなデータを持つCSV、test.csvがあったとして

```csv
user_id,email,user_name
1,a@example.com,太郎
2,b@example.com,二郎
```

次のように出力されます。

```shell
$ c2s test.csv
INSERT INTO test ( user_id, email, user_name ) VALUES ( 1, 'a@example.com', '太郎' );
INSERT INTO test ( user_id, email, user_name ) VALUES ( 2, 'b@example.com', '二郎' );
```
* ファイル名: テーブル名として扱われます。
* 1行目: カラム名として扱われます。
