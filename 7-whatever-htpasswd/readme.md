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

```txt
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

## How the old style HTTP Basic auth Works

1. **Browser requests** without credentials:

```
GET /admin/ HTTP/1.1
```

2. **Server responds** with 401 challenge:

```http
HTTP/1.1 401 Unauthorized
WWW-Authenticate: Basic realm="admin"
```

3. **Browser shows auth popup** (because of `WWW-Authenticate` header)

4. **Browser encodes credentials** in Base64 and resends:

```http
GET /admin/ HTTP/1.1
Authorization: Basic cm9vdDpxd2VydHkxMjNA
```

Base64 is **encoding only** (not encryption) - trivially reversible:
```bash
echo -n "root:qwerty123@" | base64
# cm9vdDpxd2VydHkxMjNA
```

## OWASP

- **A02:2021 - Cryptographic Failures** (MD5, unencrypted HTTP)
- **A04:2021 - Insecure Design** (exposed config files)
- **A07:2021 - Identification and Authentication Failures** (weak credentials)