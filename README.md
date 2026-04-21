# darkly-ctf

A ctf for web security

The goal is to find **14 different breaches** on a provided virtual machine, each rewarding you with a "flag". Per the subject, you must be able to explain the "how" and "why" of every exploit, making understanding the underlying concept more important than just getting the flag.


## Flags

### 1 - Member SQL Injection (Users)

Flag: 5ff9d0165b4f92b14994e5c685cdce28 (decrypted: FortyTwo)
Location: /index.php?page=member
Vulnerability: UNION-based SQL injection on user search

### 2 - Member SQL Injection (Images)

Flag: f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188
Location: /index.php?page=searchimg
Vulnerability: UNION-based SQL injection on image search with hex-encoded table names

### 3 - Broken Access Control

Flag: f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188
Location: /index.php?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f
Vulnerability: HTTP header spoofing (Referer: NSA.gov, User-Agent: ft_bornToSec)

### 4 - Open Redirects

Flag: B9E775A0291FED784A2D9680FCFAD7EDD6B8CDF87648DA647AAF4BBA288BCAB3
Location: /index.php?page=redirect&site=
Vulnerability: Unhandled invalid redirect parameter triggers flag display

### 5 - Cookie-Based Authentication Bypass
Flag: df2eb4ba34ed059a1e3e89ff4dfc13445f104a1a52295214def1c4fb1693a5c3
Location: /index.php?page=admin
Vulnerability: Client-side cookie validation (I_am_admin=MD5("true"))

### 6 - Recursive Hidden Directory
Flag: d5eec3ec36cf80dce44a896f961c1831a05526ec215693c8f2c39543497d4466
Location: /.hidden/
Vulnerability: Directory traversal through nested random-named folders

### 7 - Weak Credentials (.htpasswd)
Flag: d19b4823e0d5600ceed56d5e896ef328d7a2b9e7ac7e80f4fcdb9b10bcb3e7ff
Location: /whatever/, then basic auth to /admin/
Vulnerability: Exposed .htpasswd file with crackable MD5 password

### 8 - File Upload Vulnerability
Flag: (check flag file)
Location: /index.php?page=upload
Vulnerability: Weak file type validation + path traversal

### 9 - Path Traversal
Flag: (check flag file)
Location: /index.php?page=../../../
Vulnerability: Directory traversal via page parameter

### 10 - XSS (Stored)
Flag: 0fbb54bbf7d099713ca4be297e1bc7da0173d8b3c21c1811b916a3a86652724e
Location: /index.php?page=guestbook
Vulnerability: Stored XSS in guestbook comments (<script> tags)

### 11 - Insecure Password Reset
Flag: 1d4855f7337c0c14b6f44946872c4eb33853f40b2d54393fbe94f49f1e19bbb0
Location: /index.php?page=recover
Vulnerability: Password reset email parameter not properly validated

### 12 - IDOR (Insecure Direct Object Reference)
Flag: 03a944b434d5baff05f46c4bede5792551a2595574bcafc9a6e25f67c382ccaa
Location: /index.php?page=survey
Vulnerability: Parameter tampering (sujet=42&valeur=42)

### 13 - Sign-in Brute Force
Flag: b3a6e43ddf8b4bbb4125e5e7d23040433827759d4de1c04ea63907479a80a6b2
Location: /index.php?page=signin
Vulnerability: Weak credentials brute-forced (admin/shadow)

### 14 - Object Tag Data URI Injection
Flag: 928d819fc19405ae09921a2b71227bd9aba106f9d2d37ac412e9e5a750f1506d
Location: /index.php?page=media&src=
Vulnerability: Unsanitized `src` parameter injected into `<object data="">` tag; base64-encoded `data:` URI bypasses filter


## 1. Essential Concepts

You should start by researching the **OWASP Top 10**, which lists the most critical web security risks. Specifically, look into:

* **Injection (SQLi):** How user input can manipulate database queries.
* **Broken Authentication:** Weaknesses in how users log in or maintain sessions.
* **Cross-Site Scripting (XSS):** Injecting malicious scripts into web pages viewed by others.
* **Broken Access Control:** Gaining access to files or directories you shouldn't see.
* **Security Misconfiguration:** Using default settings or leaving "hidden" files exposed.

## 2. Recommended Toolkit

The subject forbids using automated exploitation scripts like **sqlmap**. You are expected to perform the exploits manually to prove your understanding. These tools will help you "see" what the browser is doing:

* **Intercepting Proxy (Burp Suite or OWASP ZAP):** These allow you to capture, view, and modify HTTP requests before they reach the server.
* **Browser Developer Tools:** Use the "Network" and "Inspector" tabs (F12) to see hidden fields in forms or interesting comments in the HTML source code.
* **Dirbuster / Gobuster (or similar):** Useful for "forced browsing" to find directories or files that aren't linked on the main page.
* **Command Line (curl/wget):** Sometimes interacting with the site via the terminal reveals more than a graphical browser.

## 3. Tips

* Learning a bit of **Python** or **Shell scripting** to automate the *search* (not the exploit) will save you hours.
* **Check the "Obvious":** Look for standard files that shouldn't be public, like `.htaccess`, `robots.txt`, or backup files (e.g., `.bak`, `~`).
* **Read the Metadata:** Sometimes information is hidden in image headers or within the comments of a page.
* **Documentation is Key:** Your turn-in requires a specific folder structure for each breach, including a `flag` file and a `Resources` folder containing everything needed to prove your resolution.

## Apple silicon

I used QEMU emulation because the target ISO is built for i386 (Intel 32-bit) while the hardware is ARM64 (Apple Silicon).

Make sure you have that emulator installed:

```bash
brew install qemu
which qemu-system-x86_64
```

You can run using this command:

``` bash
qemu-system-x86_64 -cdrom Darkly_i386.iso -m 1024 -cpu qemu64 -netdev user,id=net0,hostfwd=tcp::8080-:80 -device e1000,netdev=net0
```

Here is a breakdown of what each part of that command is doing:

Command Breakdown:
qemu-system-x86_64: This starts the emulator for a 64-bit Intel/AMD architecture. Even though the ISO is i386 (32-bit), this version is backward compatible and generally more stable on M1 Macs.
-cdrom Darkly_i386.iso: This tells the virtual machine to treat your Darkly_i386.iso file as a physical CD inserted into the drive.
-m 1024: This allocates 1024 MB (1 GB) of RAM to the virtual machine. This is plenty for a simple security audit project like Darkly.
-cpu qemu64: This defines the model of the CPU the VM "sees." Using a standard qemu64 profile ensures the virtual environment stays stable while emulating Intel instructions on your M1 chip.
-netdev user,id=net0,hostfwd=tcp::8080-:80. It sets up Port Forwarding:
It maps port 8080 on the Mac to port 80 (the web server) inside the VM.
To access the site: You won't use the IP shown in the VM prompt. Instead, you will open your browser and go to http://localhost:8080.
-device e1000,netdev=net0: This specifies the virtual network card (Intel E1000) for the VM to use so it can actually communicate with your Mac.

## make a local copy of the website

Make a local copy of the website to look for vunerabilities in the source code.
If you don't have it, install `wget` via brew: `brew install wget`.

```bash
wget -mk http://localhost:8080
```

What these flags do:
-m (mirror): Turns on recursion, time-stamping, and infinite depth.
-k (convert links): After the download, it changes the links in the HTML so they point to your local files instead of the server.

Or using curl (For specific pages):

```bash
curl http://localhost:8080 > index_source.html
```

While wget is great, it only finds files that are linked. In cybersecurity, many breaches are hidden in files that have no links (like a secret .hidden folder or a config.php.bak).
To find those, you might eventually need a "fuzzer" or "wordlist" tool like ffuf or gobuster (also available via brew).

## 1st Flag

On the main page click on the Members top link.

In this page: `http://localhost:8080/index.php?page=member`
you can look member by id but it is open to sql injection.
See [](1-Member_sql_injection_users/readme.md)

## 2nd Flag
in the main page html and css. Look in the html and look for flag:
I see 

```css
.fa-flag:before {
    content: "\f024"
}
.fa-flag-o:before {
    content: "\f11d"
}
.fa-flag-checkered:before {
    content: "\f11e"
}
```


| CSS Selector       | Unicode | Visual Icon     | Potential Meaning in Darkly                       |
|--------------------|---------|-----------------|---------------------------------------------------|
| .fa-flag           | \f024   | Solid Flag      | A standard flag or a "found" state.               |
| .fa-flag-o         | \f11d   | Outlined Flag   | An "unclaimed" flag or a placeholder.             |
| .fa-flag-checkered | \f11e   | Racing Flag     | The end of a sequence or a major milestone breach.|

## robots.txt

```bash
curl http://localhost:8080/robots.txt
```

```txt
User-agent: *
Disallow: /whatever
Disallow: /.hidden
```

So I went to that `/whatever` directory with `localhost:8080/whatever`

## md5 hash?

Identifying an **MD5 hash** is a common task in CTFs (Capture The Flag) like Darkly. Since these are often used as "flags" or for simple password storage, they have a very distinct "fingerprint."

Here is how you can tell if that random string you found is an MD5 hash:

### 1. The "Rule of 32"

The most reliable physical characteristic of an MD5 hash is its length. When represented in a human-readable format, an MD5 hash is **always exactly 32 characters long**.

* **Count them:** If your string is 31 or 33 characters, it is **not** a standard MD5 hash.
* **Compare to others:**
* **MD5:** 32 chars.
* **SHA-1:** 40 chars.
* **SHA-256:** 64 chars.



### 2. Hexadecimal Only

An MD5 hash is a "hexadecimal" representation of a 128-bit number. This means it will only ever contain:

* **Numbers:** `0-9`
* **Letters:** `a-f` (or `A-F`, as it's case-insensitive).
* **No other symbols:** If you see a `g`, `z`, `?`, or `$`, it's either not MD5 or it's an "encoded" or "salted" hash.

### 3. Use Identification Tools

Since you're on a Mac with Homebrew, you can use specialized tools that "guess" the hash type for you based on its pattern:

* **`hashid`**: A popular command-line tool for this purpose.

```bash
brew install hashid
hashid YOUR_HASH_STRING_HERE
```

### 4. The "Crackability" Test

In Darkly, the flag is rarely the hash itself; it's often the *result* of the hash.

* **Go to [CrackStation](https://crackstation.net/):** Paste your 32-character string.
* **The Result:** Because MD5 is cryptographically "broken" and very fast to compute, online databases have pre-computed billions of common words. If CrackStation gives you a word like `password123` or `flag_found`, you know for sure it was an MD5 hash.

### Summary Checklist

| Feature | MD5 Requirement |
| --- | --- |
| **Length** | Exactly 32 characters. |
| **Character Set** | 0-9 and a-f (hex). |
| **Entropy** | Appears completely random (no repeating patterns). |

**Once your crawler finds a 32-character hex string, try cracking it. If it works, you've officially solved your first breach! Ready to look at how to find the next one in the "Member Search" section?**

[Identification of different hash types](https://www.google.com/search?q=https://www.youtube.com/watch%3Fv%3DiVXLM-e5gkQ)
\

Open redirect looks very likely. The site exposes routes like localhost:8080/index.php?page=redirect&site=facebook, localhost:8080/index.php?page=redirect&site=twitter, and localhost:8080/index.php?page=redirect&site=instagram. That is a classic place to test parameter tampering.

File upload is a strong candidate. The upload form is visible in localhost:8080/index.php?page=upload. That usually means content-type, extension, and execution-path validation are worth checking.

File inclusion or page parameter abuse is plausible. The whole app routes through index.php?page=..., visible in localhost:8080/index.php. In challenges like this, page is often worth testing for local file inclusion or direct access to unintended files.

Hidden content / forced browsing is definitely part of the challenge set. The main README already calls out /whatever and /.hidden via README.md. That is not injection, but it is another confirmed class of breach in this VM.

XSS is probably present somewhere, but not yet confirmed by the repo notes. The main README explicitly lists XSS among the expected concepts in README.md.

So the short answer is: I do not see evidence of a third documented SQL injection here, but I do see several likely non-SQL injection-style attack surfaces. If you want the fastest next path, I would check them in this order:

page=redirect&site=...
page=upload
page= parameter tampering for inclusion
feedback/signin/search forms for reflected or stored XSS


1. What is Upgrade-Insecure-Requests: 1?
This is a header your browser sends to tell the server: "Hey, if you have an HTTPS version of this page, please send me there instead of the insecure HTTP version."

The 1 acts like a "True" flag.

In the context of Darkly, this usually doesn't do much because the CTF VM is likely running on old-school HTTP (port 80).

It's basically your browser being proactive about security, trying to avoid being "sniffed" on a public network.

2. What is detectportal.firefox.com?
This is Firefox checking for a Captive Portal.

Have you ever joined hotel or airport Wi-Fi, and a "Login/Agree" page pops up before you can use the internet? Firefox (and macOS/Windows/Android) detects that by trying to fetch a very specific, simple file from a known server (like canonical.html).

If it gets the file: It knows the internet is "open" and working.

If it gets a redirect or a 404: It knows you are trapped behind a "Captive Portal" (like a hotel login) and will pop up a notification asking you to log in.

Why are you seeing so many of them?
Because you are using a Proxy (Burp) and a VM (Darkly), Firefox is getting a bit confused. It sees that it’s not reaching the "real" internet normally, so it keeps re-checking to see if the network status has changed.


## links
https://github.com/digininja/DVWA  
https://en.wikipedia.org/wiki/Damn_Vulnerable_Web_Application. 
https://docs.metasploit.com/docs/using-metasploit/getting-started/nightly-installers.html  

https://code.claude.com/docs/en/common-workflows
