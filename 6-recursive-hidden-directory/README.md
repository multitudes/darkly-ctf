# The .hidden Directory

## Vulnerability Type

**OWASP A01:2021 - Broken Access Control** (CWE-552: Files or Directories Accessible to External Parties)

## Discovery

```txt
Disallow: /.hidden  
```

Visited the directory and found it's full of recursively nested random-named folders, each containing more folders.

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
