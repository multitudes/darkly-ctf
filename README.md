# darkly-ctf
a ctf for web security

Congratulations on starting **Darkly**! This project is a classic at 42, often serving as a wake-up call for students regarding how vulnerable "simple" web applications can be.

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

**Would you like me to help you set up a basic Python script for web scraping to help with the "deep dive" portions of the project?**