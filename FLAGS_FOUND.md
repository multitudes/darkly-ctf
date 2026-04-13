# Darkly CTF - Flags Found

Progress: 12/14 flags

## Completed Breaches

### 1 - Member SQL Injection (Users)
Flag: 5ff9d0165b4f92b14994e5c685cdce28 (decrypted: FortyTwo)
Location: /index.php?page=member
Vulnerability: UNION-based SQL injection on user search

### 2 - Member SQL Injection (Images)
Flag: f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188
Location: /index.php?page=searchimg
Vulnerability: UNION-based SQL injection on image search with hex-encoded table names

### 3 - Broken Access Control
Flag: f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188
Location: /index.php?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f
Vulnerability: HTTP header spoofing (Referer: NSA.gov, User-Agent: ft_bornToSec)

### 4 - Open Redirects
Flag: B9E775A0291FED784A2D9680FCFAD7EDD6B8CDF87648DA647AAF4BBA288BCAB3
Location: /index.php?page=redirect&site=
Vulnerability: Unhandled invalid redirect parameter triggers flag display

### 5 - Cookie-Based Authentication Bypass
Flag: df2eb4ba34ed059a1e3e89ff4dfc13445f104a1a52295214def1c4fb1693a5c3
Location: /index.php?page=admin
Vulnerability: Client-side cookie validation (I_am_admin=MD5("true"))

### 6 - Recursive Hidden Directory
Flag: d5eec3ec36cf80dce44a896f961c1831a05526ec215693c8f2c39543497d4466
Location: /.hidden/
Vulnerability: Directory traversal through nested random-named folders

### 7 - Weak Credentials (.htpasswd)
Flag: d19b4823e0d5600ceed56d5e896ef328d7a2b9e7ac7e80f4fcdb9b10bcb3e7ff
Location: /whatever/, then basic auth to /admin/
Vulnerability: Exposed .htpasswd file with crackable MD5 password

### 8 - File Upload Vulnerability
Flag: (check flag file)
Location: /index.php?page=upload
Vulnerability: Weak file type validation + path traversal

### 9 - Path Traversal
Flag: (check flag file)
Location: /index.php?page=../../../
Vulnerability: Directory traversal via page parameter

### 10 - XSS (Stored)
Flag: 0fbb54bbf7d099713ca4be297e1bc7da0173d8b3c21c1811b916a3a86652724e
Location: /index.php?page=guestbook
Vulnerability: Stored XSS in guestbook comments (<script> tags)

### 11 - Insecure Password Reset
Flag: 1d4855f7337c0c14b6f44946872c4eb33853f40b2d54393fbe94f49f1e19bbb0
Location: /index.php?page=recover
Vulnerability: Password reset email parameter not properly validated

### 12 - IDOR (Insecure Direct Object Reference)
Flag: 03a944b434d5baff05f46c4bede5792551a2595574bcafc9a6e25f67c382ccaa
Location: /index.php?page=survey
Vulnerability: Parameter tampering (sujet=42&valeur=42)

## Still Needed

Breach 13: Unknown
Breach 14: Unknown

Untested pages:
- /index.php?page=search
- /index.php?page=feedback
- /index.php?page=contact
- /index.php?page=vote
- Other parameters
