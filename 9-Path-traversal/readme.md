# Path traversal

When uploading the php file in the previous exploit i tried to execute it but always got "nope" as alert when trying a path traversal. I thought maybe i need to try somewhere else... So i did a get req in burp, the admin cookie did not matter this time but the amount of ../ did matter.

```http
GET /index.php?page=../../../../../../../etc/passwd HTTP/1.1
```

```js
alert('Congratulaton!! The flag is : b12c4b2cb8094750ae121a676269aa9e2872d07c06e429d25a63196ec1c8c1d0 ');
```

## the /etc/passwd Exploit

In Linux-based systems, /etc/passwd is a plain-text file that contains a list of every user account on the system.

Is it really a "Password" file? No, but it used to be.  
Historically: In the early days of Unix, this file did contain the encrypted password hashes.

In Modern Systems: For security, hashes were moved to a second file called /etc/shadow, which is only readable by the "root" user.

Still good to get user names, the home folders names