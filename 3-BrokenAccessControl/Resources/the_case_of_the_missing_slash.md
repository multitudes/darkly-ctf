# missing slash

## Nice job catching that! You stumbled into a classic edge case in web security challenges.

The only difference between your failing command and your winning command is the trailing slash on the Referer URL:

* **Failed:** `[https://www.nsa.gov](https://www.nsa.gov)`
* **Passed:** `[https://www.nsa.gov/](https://www.nsa.gov/)`

Here is exactly why that tiny slash made all the difference.

### 1. Hardcoded, Strict String Matching

In challenges like *Darkly*, the backend PHP code checking your headers is usually written very simply. Instead of parsing the URL properly using robust built-in functions, the developer likely used a strict string comparison like this:

```php
if ($_SERVER['HTTP_REFERER'] === "https://www.nsa.gov/") {
    // Give flag
}

```

Because it expects a strict, exact character match, `[https://www.nsa.gov](https://www.nsa.gov)` (without the slash) fails the check entirely. To the code, missing that single character is just as bad as typing `google.com`.

### 2. How Browsers Handle Root Paths

In the real world, if you are sitting on the home page of `nsa.gov` and click a link to go somewhere else, your web browser automatically generates the `Referer` header.

According to official web standards (RFC 7230), an absolute URI pointing to a server root *must* include a hierarchical path. If there isn't one, the browser appends a `/`. Therefore, a real browser leaving the NSA homepage will almost always send `Referer: [https://www.nsa.gov/](https://www.nsa.gov/)`.

The CTF creator likely just copied a real browser header value when hardcoding the solution verification on the server side, making the trailing slash mandatory.

### A Quick Tip for `curl` Debugging

When you are dealing with header-based challenges and things aren't working, remove the `| grep flag` pipe and add `-v` (verbose) or `-i` (include headers) to your curl command:

```bash
curl -i -H "Referer: ..." ...

```

Often, the server will send back subtle clues in the HTML body or the HTTP response headers (like a `403 Forbidden` or a custom comment) telling you exactly *which* condition you failed to meet!
