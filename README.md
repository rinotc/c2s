# c2s

CSVファイルから、SQLのINSERT文をgenerateするツール

## How to install

### install via cargo

```shell
cargo install c2s
```

## How to use

以下のようなデータを持つCSV、users.csvがあったとして

```csv
user_id,email,user_name,height,weight,birthday
1,a@example.com,太郎,172.5,null,2022-05-05
2,b@example.com,二郎,182.3,92.03,null
```

次のように出力されます。

```shell
$ c2s users.csv
INSERT INTO users ( user_id, email, user_name, height, weight, birthday ) VALUES ( 1, 'a@example.com', '太郎', 172.5, null, '2022-05-05' );
INSERT INTO users ( user_id, email, user_name, height, weight, birthday ) VALUES ( 2, 'b@example.com', '二郎', 182.3, 92.03, null );
```
* ファイル名: テーブル名として扱われます。
* 1行目: カラム名として扱われます。
* 明示的に `null` と書かれているものを`null`と出力します。何もないところには、`tanaka,,65.0` のような部分は `VALUES ( ...,'tanaka','',65.0 )` と出力されます。

また、明示的にテーブル名を指定することもできます。

```shell
$ c2s users.csv demo_users
INSERT INTO demo_users ( user_id, email, user_name, height, weight, birthday ) VALUES ( 1, 'a@example.com', '太郎', 172.5, null, '2022-05-05' );
INSERT INTO demo_users ( user_id, email, user_name, height, weight, birthday ) VALUES ( 2, 'b@example.com', '二郎', 182.3, 92.03, null );
```


