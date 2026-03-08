# darkly-ctf
a ctf for web security

# Apple silicon

I used QEMU emulation because the target ISO is built for i386 (Intel 32-bit) while your hardware is ARM64 (Apple Silicon).

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

Command Breakdown
qemu-system-x86_64: This starts the emulator for a 64-bit Intel/AMD architecture. Even though the ISO is i386 (32-bit), this version is backward compatible and generally more stable on M1 Macs.
-cdrom Darkly_i386.iso: This tells the virtual machine to treat your Darkly_i386.iso file as a physical CD inserted into the drive.
-m 1024: This allocates 1024 MB (1 GB) of RAM to the virtual machine. This is plenty for a simple security audit project like Darkly.
-cpu qemu64: This defines the model of the CPU the VM "sees." Using a standard qemu64 profile ensures the virtual environment stays stable while emulating Intel instructions on your M1 chip.
-netdev user,id=net0,hostfwd=tcp::8080-:80: This is the most important part for you. It sets up Port Forwarding:
It maps port 8080 on your Mac to port 80 (the web server) inside the VM.
To access the site: You won't use the IP shown in the VM prompt. Instead, you will open your browser and go to http://localhost:8080.
-device e1000,netdev=net0: This specifies the virtual network card (Intel E1000) for the VM to use so it can actually communicate with your Mac.

## make a local copy of the website 

to look for vunerabilities in the source code.
Use `wget`.
If you don't have it, install it via brew: `brew install wget`.

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

The goal is to find **14 different breaches** on a provided virtual machine, each rewarding you with a "flag". Per the subject, you must be able to explain the "how" and "why" of every exploit, making understanding the underlying concept more important than just getting the flag.

### 1. Essential Concepts (The "What to Study")

The project explicitly mentions **OWASP** (Open Web Application Security Project). You should start by researching the **OWASP Top 10**, which lists the most critical web security risks. Specifically, look into:

* **Injection (SQLi):** How user input can manipulate database queries.
* **Broken Authentication:** Weaknesses in how users log in or maintain sessions.
* **Cross-Site Scripting (XSS):** Injecting malicious scripts into web pages viewed by others.
* **Broken Access Control:** Gaining access to files or directories you shouldn't see.
* **Security Misconfiguration:** Using default settings or leaving "hidden" files exposed.

### 2. Recommended Toolkit

The subject forbids using automated exploitation scripts like **sqlmap**. You are expected to perform the exploits manually to prove your understanding. These tools will help you "see" what the browser is doing:

* **Intercepting Proxy (Burp Suite or OWASP ZAP):** These allow you to capture, view, and modify HTTP requests before they reach the server.
* **Browser Developer Tools:** Use the "Network" and "Inspector" tabs (F12) to see hidden fields in forms or interesting comments in the HTML source code.
* **Dirbuster / Gobuster (or similar):** Useful for "forced browsing" to find directories or files that aren't linked on the main page.
* **Command Line (curl/wget):** Sometimes interacting with the site via the terminal reveals more than a graphical browser.

### 3. Tips for Your Mindset

* **Be a "Scraper":** One of the challenges involves sifting through thousands of files. Learning a bit of **Python** or **Shell scripting** to automate the *search* (not the exploit) will save you hours.
* **Check the "Obvious":** Look for standard files that shouldn't be public, like `.htaccess`, `robots.txt`, or backup files (e.g., `.bak`, `~`).
* **Read the Metadata:** Sometimes information is hidden in image headers or within the comments of a page.
* **Documentation is Key:** Your turn-in requires a specific folder structure for each breach, including a `flag` file and a `Resources` folder containing everything needed to prove your resolution.


## 1st Flag

On the main page click on `© BornToSec` on the bottom. The link opens to a weird page with audio. Then click on the Members top link and enter 5:

[](http://localhost:8080/?page=member&id=5&Submit=Submit#)

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

