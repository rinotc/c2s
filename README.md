# c2s

CSVファイルから、SQLのINSERT文をgenerateするツール

## example

test.csv

```csv
user_id,email,user_name
1,a@example.com,太郎
2,b@example.com,二郎
```

が、次のように変換されます。

```sql
INSERT INTO test ( user_id, email, user_name ) VALUES ( 1, 'a@example.com', '太郎' );
INSERT INTO test ( user_id, email, user_name ) VALUES ( 2, 'b@example.com', '二郎' );
```
* ファイル名: テーブル名として扱われます。
* 1行目: カラム名として扱われます。