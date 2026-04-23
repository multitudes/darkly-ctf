# Open Redirects

## Vulnerability Type
**OWASP A04:2021 - Insecure Design** (CWE-601: URL Redirection to Untrusted Site)

## Summary

page: `http://localhost:8081/index.php?page=redirect&site=`
commands:
```bash
curl -i "http://localhost:8081/index.php?page=redirect&site=anything_invalid"
```
Just change `site=` to any value not in the whitelist (facebook, twitter, instagram).

flag is B9E775A0291FED784A2D9680FCFAD7EDD6B8CDF87648DA647AAF4BBA288BCAB3
## Typical example:

```txt
index.php?page=redirect&site=facebook
```

If the server only checks “take the `site` parameter and redirect there”, an attacker may be able to turn it into:

```txt
index.php?page=redirect&site=https://evil.example
```

index.php?page=redirect&site=https://www.example.com

Why that is dangerous:

1. It makes a malicious link look trustworthy.
   The victim sees your legitimate domain first, clicks it, and then gets silently sent somewhere else.

2. It is useful for phishing.
   A link starting with your real site is much more convincing in emails, chats, or fake login flows.

3. It can help bypass filters.
   Some systems trust your domain and may allow the first URL, even though it ends up elsewhere.

4. It can chain with other bugs.
   Open redirects are often combined with OAuth issues, token leakage, password reset flows, or XSS-style tricks.

So the core issue is not “the page moves somewhere else”. The issue is: your site becomes a trusted jumping point for attackers.

A safe implementation usually does one of these:

- use a strict allowlist such as only `facebook`, `twitter`, `instagram`
- map friendly names to hardcoded URLs on the server
- reject full external URLs from user input

## The Vulnerability

The problem with an open redirect is that the application lets user-controlled input decide where the browser goes next.

I tried to change the redirect and it did not redirect. Instead it gave me the flag!

```txt
GOOD JOB HERE IS THE FLAG : B9E775A0291FED784A2D9680FCFAD7EDD6B8CDF87648DA647AAF4BBA288BCAB3
```

## How We Found It

The footer contains three safe redirect links:
```html
<li><a href="index.php?page=redirect&site=facebook" class="icon fa-facebook"></a></li>
<li><a href="index.php?page=redirect&site=twitter" class="icon fa-twitter"></a></li>
<li><a href="index.php?page=redirect&site=instagram" class="icon fa-instagram"></a></li>
```

These work fine (HTTP 302 redirect). But when we try an invalid site:
```bash
curl -i "http://localhost:8080/index.php?page=redirect&site=instagramssss"
```

Instead of redirecting or erroring, the server returns the flag!

## The Bug

The backend probably has logic like this:
```php
if ($site == 'facebook' || $site == 'twitter' || $site == 'instagram') {
    // Safe redirects - do proper redirect
    redirect_to(get_approved_url($site));
} else {
    // Oops! They forgot to handle invalid input properly
    // Instead of safely rejecting it, this shows debug/flag info
    echo "Good job here is the flag...";
}
```

## Remediation
1. **Whitelist allowed destinations** – Only permit explicitly approved redirect targets
2. **Validate URL scheme** – Reject URLs starting with `//`, `http://`, `https://` from user input
3. **Use relative URLs only** – If redirecting, keep redirects within your domain
4. **Validate domain** – If external redirects are needed, check the redirect target's domain against a whitelist
5. **Log redirect attempts** – Monitor suspicious redirect patterns for phishing or abuse detection
```
