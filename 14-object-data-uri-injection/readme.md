# Breach #14: Object Tag Injection via `data:` URI

## Flag
928d819fc19405ae09921a2b71227bd9aba106f9d2d37ac412e9e5a750f1506d

## Summary

page: `http://localhost:8080/index.php?page=media&src=`
commands:
```bash
# Encode the payload
echo -n '<script>alert(1)</script>' | base64
# → PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==

# Inject via data: URI with base64
curl "http://localhost:8080/index.php?page=media&src=data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==" | grep flag
```

## Vulnerability Type

**OWASP A03:2021 - Injection** (CWE-79: Cross-Site Scripting via Object Tag)

## Location

```
http://localhost:8081/index.php?page=media&src=
```

The homepage links to `?page=media&src=nsa`, which embeds an image using an `<object data="...">` tag. The `src` parameter controls what gets loaded inside that object tag.

## How It Works

1. **Discovery**: On the homepage, there is a link to the media page:
   ```html
   <a href="?page=media&src=nsa">
   ```

2. **Normal behavior**: With `src=nsa`, the server renders:
   ```html
   <object data="http://10.0.2.15/images/nsa_prism.jpg"></object>
   ```
   It maps the short name `nsa` to the image file `nsa_prism.jpg`.

3. **The vulnerability**: The `src` parameter is injected directly into the `<object data="...">` attribute without proper validation. This allows injecting a `data:` URI to embed arbitrary HTML/JavaScript.

4. **Filter bypass**: A raw `data:text/html,<script>...</script>` payload gets detected and returns "WrongAnswer". However, **base64 encoding** the payload bypasses the filter entirely.

## Exploit

```bash
curl -s "http://localhost:8081/index.php?page=media&src=data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==" | grep flag
```

Where the base64 payload decodes to:
```html
<script>alert(1)</script>
```

### Encoding the payload

```bash
echo -n '<script>alert(1)</script>' | base64
# Output: PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==
```

### What happens

| Payload | Result |
|---|---|
| `src=nsa` | Normal image displayed |
| `src=data:text/html,<script>alert(1)</script>` | ❌ "WrongAnswer" — raw HTML is filtered |
| `src=data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==` | ✅ Flag revealed — base64 bypasses the filter |

## Why It's Dangerous

- The `<object>` tag can load arbitrary content types (HTML, SVG, Flash, etc.)
- `data:` URIs allow embedding inline content without fetching from a server
- Base64 encoding is a trivial way to bypass naive input sanitization
- An attacker could use this to execute JavaScript in the context of the victim's session (session hijacking, cookie theft, etc.)

## Remediation

1. **Whitelist allowed `src` values** — Only accept known, predefined values (`nsa`, `prism`, etc.) rather than passing raw input to the `data` attribute
2. **Never embed user input in `<object data="">`** — This is equivalent to injecting into an `<iframe src="">`
3. **Validate URI scheme** — Block `data:`, `javascript:`, and `vbscript:` URI schemes
4. **Content Security Policy (CSP)** — Use `object-src 'none'` to prevent object tag abuse entirely
5. **Output encoding** — HTML-encode all user input before inserting it into HTML attributes

======================

i have to admit i am not sure i understand the data - object fully 
the link on the homeage is 
<a href="?page=media&src=nsa"><img src="images/nsa_prism.jpg" alt="" /></a>

following the link 
http://localhost:8081/index.php?page=media&src=nsa

i only get 
File: nsa_prism.jpg

but no image. what that mean?

>>>>>>>>>>>>>>>>

Great question — let me break it down step by step.

What's happening with page=media&src=nsa
When you visit that URL, the server generates this HTML:

```html
<object data="http://10.0.2.15/images/nsa_prism.jpg"></object>
```
The server takes your src=nsa input and maps it to nsa_prism.jpg, then sticks it inside an <object data="..."> tag.

Why you see "File: nsa_prism.jpg" but no image: The <object> tag tries to load from http://10.0.2.15/images/nsa_prism.jpg — that's the VM's internal IP. Since you're accessing the VM via localhost:8081 (port-forwarded), your browser can't reach 10.0.2.15 directly. So the object fails to load, and you only see the text label.

The <object> tag — why it's dangerous
The <object> tag is like a powerful <iframe>. It can embed anything — images, HTML pages, PDFs, even scripts. The key attribute is data=, which tells it what to load.

Think of it as:

<object data="[ANYTHING YOU WANT TO LOAD]"></object>
The exploit — step by step
1. Normal use
src=nsa  →  server generates  →  <object data="http://10.0.2.15/images/nsa_prism.jpg">
Just loads an image. Harmless.

2. What if we control data=?
Since the server puts our src value into the data attribute, what if instead of nsa, we pass in a data: URI?

A data: URI is a way to embed content inline instead of fetching it from a URL:

data:text/html,<h1>Hello</h1>
This doesn't load a file from anywhere — the content IS the URL. It's like writing a tiny webpage right inside the address bar.

3. First attempt — raw HTML (blocked ❌)
src=data:text/html,<script>alert(1)</script>
Server sees <script> in the input → rejects it → shows "WrongAnswer.gif" and the alert(1) but no flag!!!

4. The bypass — base64 encoding (works ✅)
Base64 is just a way to represent data as letters/numbers. The server's filter looks for <script> in the raw text, but it doesn't decode base64 to check:

bash
# This is <script>alert(1)</script> encoded as base64:

```
echo -n '<script>alert(1)</script>' | base64
# → PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==
```

So we send:

src=data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==
The server generates:

```html
<object data="data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg=="></object>
The server's filter sees no <script> tag (it's hidden in the base64 gibberish), so it accepts it and returns the flag.
```

TL;DR
Your input (src=)  →  Goes into <object data="YOUR INPUT">  →  Browser executes it
nsa                →  Loads an image (normal use)
data:text/html,... →  Loads arbitrary HTML/JS (the vulnerability)
base64 encoding    →  Hides the payload from the server's filter (the bypass)
The core issue: the server trusts user input and puts it directly into a dangerous HTML attribute without proper validation. It tries to filter <script> tags, but base64 encoding trivially defeats that.



