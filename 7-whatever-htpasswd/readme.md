# Exposed .htpasswd File & Weak Credentials

## Vulnerability Type
**OWASP A02:2021 - Cryptographic Failures** (CWE-326: Inadequate Encryption Strength)
**OWASP A07:2021 - Identification and Authentication Failures** (CWE-521: Weak Password Requirements)
**OWASP A01:2021 - Broken Access Control** (CWE-276: Incorrect Default Permissions)

Found `/whatever/` directory (listed in robots.txt). Inside was an exposed `.htpasswd` file containing:
```
root:437394baff5aa33daa618be47b75cb49
```

This is Apache's password file format. The hash is an MD5 hash.

## Credential Extraction

Cracked the MD5 hash on crackstation.net → `qwerty123@`

Now have credentials: `root:qwerty123@`

## Actual Exploitation

Tried `/admin/` directory. 

Browser showed auth popup. Entered credentials. Got access and flag:
```
d19b4823e0d5600ceed56d5e896ef328d7a2b9e7ac7e80f4fcdb9b10bcb3e7ff
```

## Key Failures

1. **Exposed .htpasswd in web-accessible directory** – Configuration files must never be web-facing
2. **MD5 hashing for passwords** – MD5 is cryptographically broken (not salted, fast to crack)
3. **Weak credentials** – Simple dictionary password `qwerty123@` easily cracked
4. **HTTP (not HTTPS)** – Credentials transmitted in plaintext

## Remediation
1. **Secure file storage** – Never store `.htpasswd`, `.env`, or config files in document root
2. **Use strong password hashing** – Replace MD5 with bcrypt, scrypt, Argon2, or PBKDF2 with high work factors
3. **Enforce strong passwords** – Minimum 12 characters, complexity requirements, no dictionary words
4. **Use HTTPS only** – Encrypt all credentials in transit (HSTS headers)
5. **Rotate credentials** – Change default/backup credentials regularly
6. **Restrict directory access** – Use `.htaccess` or web server ACLs to block external access to config directories
7. **Monitor failed auth attempts** – Alert on brute force patterns