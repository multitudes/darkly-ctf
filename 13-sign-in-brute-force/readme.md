# Brute force the sign in?

## Summary

page: `http://localhost:8080/index.php?page=signin`
commands:
```bash
curl "http://localhost:8080/index.php?page=signin&username=admin&password=shadow&Login=Login"
```
Credentials: `admin` / `shadow` (brute-forced).

## Details

on the sign in page

Used "admin" as the username
to brute force the password... with this script



So i got the flag:
```
http://localhost:8081/index.php?page=signin&username=admin&password=shadow&Login=Login#
```