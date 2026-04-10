# whatever

In `robots.txt` I found the `/whatever/` directory.
Going to the page I found the /whatever/htpasswd file which I downloaded.
it contains the root password as md5 hash. so i have the password.

So next is to go to the signin page:

```txt
http://localhost:8080/?page=signin
```

I used root / qwerty123@ on the signin page but got wrong answer

I tried the /admin route but the server dropped the connection. this made me suspicious because it is not the usual 404. I though maybe i need to enter the login as query? like this which is a basic form of authentication:

http://root:PASSWORD@localhost:8080/admin/

encoding the `@` as `%40` my url is:
http://root:qwerty123%40@localhost:8080/admin/

and I got the auth box popping up. I had to reenter the credentials and got the flag:
d19b4823e0d5600ceed56d5e896ef328d7a2b9e7ac7e80f4fcdb9b10bcb3e7ff

When you hit `http://root:qwerty123%40@localhost:8080/admin/`, modern browsers **strip credentials from the URL** before sending the request (security policy). So the flow was:

**1. Browser sends request without credentials:**

```
GET /admin/ HTTP/1.1
Host: localhost:8080
```

**2. Server responds 401, challenging for credentials:**
```
HTTP/1.1 401 Unauthorized
WWW-Authenticate: Basic realm="admin"
```

**3. Browser shows the auth box** (because of that `WWW-Authenticate` header). You type `root` / `qwerty123@`.

**4. Browser encodes `root:qwerty123@` in Base64** → `cm9vdDpxd2VydHkxMjNA` and resends:
```
GET /admin/ HTTP/1.1
Host: localhost:8080
Authorization: Basic cm9vdDpxd2VydHkxMjNA
```

The Base64 is just encoding, not encryption — trivially reversible:
```bash
echo -n "root:qwerty123@" | base64
# cm9vdDpxd2VydHkxMjNA
```

That's why HTTP Basic Auth over plain HTTP is a critical security issue — anyone sniffing the network can decode the credentials instantly. It must be used over HTTPS only.

 the first request has no Authorization header. That's by design — HTTP Basic Auth is a challenge-response protocol:

Client doesn't know auth is required upfront, so it asks with no credentials
Server says "I need credentials" via 401 + WWW-Authenticate
Only then does the client send credentials