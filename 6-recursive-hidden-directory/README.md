# The .hidden Directory

## Vulnerability Type

**OWASP A01:2021 - Broken Access Control** (CWE-552: Files or Directories Accessible to External Parties)

## Summary

page: `http://localhost:8081/.hidden/`
commands:
```bash
# Run the recursive scraper
uv run --with requests --with beautifulsoup4 scraper.py
```
No curl one-liner — requires the Python scraper to recursively crawl hundreds of nested directories.

trying with wget fails because wget reads the robot.txt and does not crawl the .hidden directory. Still can be run with 
```bash
# -r = recursive (follow links into subdirectories)
# -np = no-parent (stay inside /.hidden/, don't go up)
# -e robots=off = ignore robots.txt rules
# -P = save files to this directory
wget -r -np -e robots=off http://localhost:8081/.hidden/ -P /tmp/hidden_mirror
# then 
grep -r "flag" /tmp/hidden_mirror
```
should take 3 minutes to finish. Plus 4 secs for grep.

## Discovery in robot.txt

```txt
User-agent: *
Disallow: /whatever
Disallow: /.hidden
```

Visited the .hidden directory and found it's full of recursively nested random-named folders, each containing more folders.

```txt
http://localhost:8080/.hidden/
```

Looks like:

```txt
Index of /.hidden/

../
amcbevgondgcrloowluziypjdh/                        29-Jun-2021 18:15                   -
bnqupesbgvhbcwqhcuynjolwkm/                        29-Jun-2021 18:15                   -
ceicqljdddshxvnvdqzzjgddht/                        29-Jun-2021 18:15                   -
... (many more)
README       
```

## The Problem

Too many folders to click through manually. Wrote a Python scraper to recursively crawl all the nested directories and look for the flag.

Run with:

```bash
uv run --with requests --with beautifulsoup4 scraper.py
```

The scraper took a few minutes to finish traversing the entire structure.

## Result

```txt
Hey, here is your flag : d5eec3ec36cf80dce44a896f961c1831a05526ec215693c8f2c39543497d4466
```

## Remediation

1. **Respect robots.txt by default** – Don't create "hidden" directories expecting security through obscurity; use actual access controls
2. **Disable directory listing** – Prevent `Index of /` pages (set `Options -Indexes` in Apache or equivalent in nginx)
3. **Require authentication** – Use `.htaccess` or web server auth for sensitive directories
4. **Use proper access control** – Check authorization on every request, not just rely on hidden paths
5. **Remove admin/sensitive files from web root** – Configuration files, backups, and credentials should never be web-accessible
6. **Monitor suspicious directory patterns** – Alert on attempts to enumerate directories or traverse deep folder structures
