# DARKLY

A CTF 42 project for web security. Part of the outer core curriculum.

![darkly_header.jpg](darkly_header.jpg)


The goal is to find **14 different breaches** on a provided virtual machine, each rewarding you with a "flag". Per the subject, you must be able to explain the "how" and "why" of every exploit, making understanding the underlying concept more important than just getting the flag.

## The 14 Flags (Vulnerabilities Index)
Below is the list of all 14 exploits covered in this project. Click on each link to read the detailed breakdown and methodology for finding the flag.

1.  **[SQL Injection (Users)](./1-Member_sql_injection_users/Resources/readme.md)** - Exploiting SQL injection vulnerabilities to extract user database records.
2.  **[SQL Injection (Images)](./2-Member_sql_injection_images/Resources/readme.md)** - Manipulating database queries through image-related inputs.
3.  **[Broken Access Control](./3-BrokenAccessControl/Resources/readme.md)** - Bypassing access controls to gain unauthorized privileges.
4.  **[Open Redirects](./4-open_redirects/Resources/readme.md)** - Exploiting unvalidated redirects and forwards to malicious sites.
5.  **[Cookie-Based Auth Bypass](./5-Cookie-based-auth-bypass/Resources/readme.md)** - Manipulating browser cookies to bypass authentication mechanisms.
6.  **[Recursive Hidden Directory](./6-recursive-hidden-directory/Resources/README.md)** - Discovering hidden directories and files through recursive scraping.
7.  **[Htpasswd Bypass](./7-whatever-htpasswd/Resources/readme.md)** - Extracting or bypassing `.htpasswd` basic authentication.
8.  **[File Upload Vulnerability](./8-file-upload-vulnerability/Resources/readme.md)** - Exploiting insecure file upload handling to execute malicious code.
9.  **[Path Traversal](./9-Path-traversal/Resources/readme.md)** - Accessing restricted directories and files outside the web root.
10. **[Cross-Site Scripting (XSS)](./10-XSS/Resources/readme.md)** - Injecting malicious client-side scripts into web pages.
11. **[Forgot My Password Flaw](./11-forgot-my-password/Resources/readme.md)** - Exploiting logical flaws in the password reset mechanism.
12. **[Survey Vulnerability](./12-survey/Resources/readme.md)** - Manipulating survey or form submission logic.
13. **[Sign-in Brute Force](./13-sign-in-brute-force/Resources/readme.md)** - Demonstrating brute-force attacks on the login portal.
14. **[Object Data URI Injection](./14-object-data-uri-injection/Resources/readme.md)** - Injecting malicious payloads via data URIs in object tags.

---

## Getting Started

To start the vm contained in the iso file on the linux system on `localhost:8081`:  

```bash
qemu-system-x86_64 -m 1G -drive file=Darkly_i386.iso,format=raw,media=cdrom -boot d -net nic -net user,hostfwd=tcp::8081-:80

```

If on a Mac with Apple Silicon, you can use the following command:

```bash
qemu-system-x86_64 -cdrom Darkly_i386.iso -m 1024 -cpu qemu64 -netdev user,id=net0,hostfwd=tcp::8081-:80 -device e1000,netdev=net0

```

## Which SQL dialect?

Based on the syntax you used, you are interacting with a **MySQL** (or **MariaDB**, which is a drop-in fork of MySQL) database.

Here is the breakdown of the specific clues in your payload that give it away:

### 1. The `database()` Function

In MySQL, `database()` is a built-in function used to return the name of the current database. While other SQL dialects have similar functions, they use different names:

* **MySQL / MariaDB:** `database()`
* **PostgreSQL:** `current_database()`
* **Microsoft SQL Server:** `DB_NAME()`
* **Oracle:** `SELECT global_name FROM global_name;`

### 2. The `information_schema` Architecture

The `information_schema.tables` and `information_schema.columns` structures are part of the ANSI SQL standard, so you will find them in PostgreSQL and SQL Server as well. However, combined with the other clues, it perfectly aligns with MySQL's standard information gathering patterns.

### 3. Hex Encoding for String Literals (`0x7573657273`)

This is the ultimate giveaway. You used `0x7573657273` to represent the string `"users"`.

* **MySQL** natively interprets `0x...` hex literals as strings or numbers depending on the context, allowing you to bypass quotes entirely (which is incredibly useful if the application filters out single or double quotes).
* **PostgreSQL** does not support `0x` hex notation for string literals like this (it expects `E'\x...'` or specific functions).
* **SQLite** accepts `X'7573657273'`, but its system tables are completely different (it uses `sqlite_master`, not `information_schema`).

## 🔗 Useful Links & Tools

Tools like Burp Suite and ZAP are highly recommended! It is a good idea to learn to use those tools. They allow you to see requests and responses clearly and make discovery much easier than just using a curl from your command line.

* [Burp Suite Community Edition](https://portswigger.net/burp/communitydownload)
* [OWASP ZAP](https://www.zaproxy.org/download/)

**To practice further:**

* [Damn Vulnerable Web Application (DVWA) GitHub](https://github.com/digininja/DVWA)
* [DVWA Wikipedia](https://en.wikipedia.org/wiki/Damn_Vulnerable_Web_Application)
* [Metasploit Getting Started Guide](https://docs.metasploit.com/docs/using-metasploit/getting-started/nightly-installers.html)
