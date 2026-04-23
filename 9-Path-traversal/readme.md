# Path Traversal

## Vulnerability Type

**OWASP A01:2021 - Broken Access Control** (CWE-22: Improper Limitation of a Pathname to a Restricted Directory)

## Summary

Commands:
```bash
curl "http://localhost:8081/index.php?page=../../../../../../../etc/passwd"
```

Flag: `b12c4b2cb8094750ae121a676269aa9e2872d07c06e429d25a63196ec1c8c1d0`

## Details

When uploading the php file in the previous exploit i tried to execute it but always got "nope" as alert when trying a path traversal. I thought maybe i need to try somewhere else... So i did a get req in burp, the admin cookie did not matter this time but the amount of ../ did matter.

```http
GET /index.php?page=../../../../../../../etc/passwd HTTP/1.1
```

```js
alert('Congratulaton!! The flag is : b12c4b2cb8094750ae121a676269aa9e2872d07c06e429d25a63196ec1c8c1d0 ');
```

## the /etc/passwd Exploit

In Linux-based systems, /etc/passwd is a plain-text file that contains a list of every user account on the system.

Is it a Password file? No, but it used to be.  
Historically: In the early days of Unix, this file did contain the encrypted password hashes.

In Modern Systems: For security, hashes were moved to a second file called /etc/shadow, which is only readable by the "root" user.

Still good to get user names, the home folders names etc.

## How It Works

The `page` parameter accepts arbitrary file paths. The server does not validate or sanitize the input:

1. **Normal use**: `page=profile` → loads `pages/profile.php`
2. **Path traversal**: `page=../../../etc/passwd` → loads `/etc/passwd` from the filesystem
3. The `../` sequences escape the intended `pages/` directory and access files outside the web root

## Key Failures

1. **No input validation** – Accepts `../` sequences without filtering
2. **No path normalization** – Doesn't resolve or restrict to a whitelist directory
3. **Over-reliance on obscurity** – Assumes attackers won't try path traversal
4. **Information disclosure** – Exposes `/etc/passwd` with usernames and system info

## Remediation

1. **Whitelist allowed pages** – Only accept predefined page names (`profile`, `settings`, `help`, etc.)
2. **Reject path traversal sequences** – Block `..`, `./`, absolute paths, and URL-encoded variants (`%2e%2e`)
3. **Use realpath() validation** – Resolve the final path and verify it's within the allowed directory:
```php
$requested = realpath($pages_dir . '/' . $_GET['page']);
if (strpos($requested, $pages_dir) !== 0) {
    die('Access denied');
}
```

4. **Never concatenate user input into file paths** – Use a mapping/lookup table instead

5. **Principle of least privilege** – Web server should only have read access to intended directories

6. **Monitor file access patterns** – Alert on attempts to access /etc/, /sys/, or other sensitive locations