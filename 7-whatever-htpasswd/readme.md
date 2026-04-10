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

## First Attempt (Failed)

Tried the signin page:
```
http://localhost:8080/?page=signin
```

Entered `root` / `qwerty123@` → didn't work. Wrong endpoint.

## Actual Exploitation

Tried `/admin/` directory. Server didn't return 404 - it dropped the connection. Suspicious.

After research, realized `/admin/` requires HTTP Basic Authentication. Put credentials in URL:
```
http://root:qwerty123%40@localhost:8080/admin/
```

Browser showed auth popup. Entered credentials. Got access and flag:
```
d19b4823e0d5600ceed56d5e896ef328d7a2b9e7ac7e80f4fcdb9b10bcb3e7ff
```

## How It Works

Modern browsers strip credentials from URLs (security policy), so the flow is:

1. **Browser requests** without credentials:
```
GET /admin/ HTTP/1.1
```

2. **Server responds** with 401 challenge:
```
HTTP/1.1 401 Unauthorized
WWW-Authenticate: Basic realm="admin"
```

3. **Browser shows auth popup** (because of `WWW-Authenticate` header)

4. **Browser encodes credentials** in Base64 and resends:
```
GET /admin/ HTTP/1.1
Authorization: Basic cm9vdDpxd2VydHkxMjNA
```

Base64 is **encoding only** (not encryption) - trivially reversible:
```bash
echo -n "root:qwerty123@" | base64
# cm9vdDpxd2VydHkxMjNA
```

## Why It's Vulnerable

1. **Exposed credentials file** - `.htpasswd` shouldn't be web-accessible
2. **Weak password** - MD5 is crackable (should use bcrypt/scrypt)
3. **HTTP not HTTPS** - Basic Auth over HTTP exposes credentials to network sniffing
4. **Default/predictable username** - "root" is obvious

## OWASP

- **A02:2021 - Cryptographic Failures** (MD5, unencrypted HTTP)
- **A04:2021 - Insecure Design** (exposed config files)
- **A07:2021 - Identification and Authentication Failures** (weak credentials)