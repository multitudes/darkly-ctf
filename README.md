# darkly-ctf

A ctf 42 project for web security. Part of the outer core curriculum.

The goal is to find **14 different breaches** on a provided virtual machine, each rewarding you with a "flag". Per the subject, you must be able to explain the "how" and "why" of every exploit, making understanding the underlying concept more important than just getting the flag.

To start the vm contained in the iso file on the linus system on `localhost:8081`:  

```bash
qemu-system-x86_64 -m 1G -drive file=Darkly_i386.iso,format=raw,media=cdrom -boot d -net nic -net user,hostfwd=tcp::8081-:80
```

If on a mac with apple silicon, you can use the following command:  

```bash
qemu-system-x86_64 -cdrom Darkly_i386.iso -m 1024 -cpu qemu64 -netdev user,id=net0,hostfwd=tcp::8081-:80 -device e1000,netdev=net0
```

## which sql dialect?

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


## links
 
Tools like Burp Suite and ZAP. It is a good to learn to use those tools! They allowed to see clearly the requests and responses and make the discovery easier than just use a curl from your command line!  
https://portswigger.net/burp/communitydownload  
https://www.zaproxy.org/download/  

To practice:  
Damn Vulnerable Web Application:  
https://github.com/digininja/DVWA   
https://en.wikipedia.org/wiki/Damn_Vulnerable_Web_Application    
https://docs.metasploit.com/docs/using-metasploit/getting-started/  nightly-installers.html  

