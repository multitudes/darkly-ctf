# Breach #7: Exposed .htpasswd File & Weak Credentials

## Discovery

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

## OWASP

- **A02:2021 - Cryptographic Failures** (MD5, unencrypted HTTP)
- **A04:2021 - Insecure Design** (exposed config files)
- **A07:2021 - Identification and Authentication Failures** (weak credentials)