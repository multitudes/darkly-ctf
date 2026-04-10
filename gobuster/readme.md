# gobuster on admin

## Install gobuster

I could use brew but on their github page:
https://github.com/OJ/gobuster?tab=readme-ov-file

they recommend:

```bash
go install github.com/OJ/gobuster/v3@latest
```

Then I need some word lists.

### Recommended Wordlists

- **SecLists**: [https://github.com/danielmiessler/SecLists](https://github.com/danielmiessler/SecLists)
- **FuzzDB**: [https://github.com/fuzzdb-project/fuzzdb](https://github.com/fuzzdb-project/fuzzdb)
- **Seclists DNS**: [https://github.com/danielmiessler/SecLists/tree/master/Discovery/DNS](https://github.com/danielmiessler/SecLists/tree/master/Discovery/DNS)

---

## Using gobuster with a page needing basic auth:

This is not good

```bash
PASSWORD="qwerty123@" && gobuster dir -u http://root:PASSWORD@localhost:8080/admin/ -w common.txt
```

this better

```bash
gobuster dir -u "http://localhost:8080/admin/" -w common.txt \
  -U root -P 'qwerty123@' --exclude-length 975
```
