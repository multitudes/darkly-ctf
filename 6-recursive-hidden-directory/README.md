# The .hidden Directory

Found in robots.txt:
```
Disallow: /.hidden  
```

Visited the directory and found it's full of recursively nested random-named folders, each containing more folders.

```
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

```
Hey, here is your flag : d5eec3ec36cf80dce44a896f961c1831a05526ec215693c8f2c39543497d4466
```
