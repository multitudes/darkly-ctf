
# the .hidden directory

found in the robots.txt file

```text
Disallow: /.hidden  
```

so i went there and found a long ist of recursively created random links which pages of more random links.

```txt
http://localhost:8080/.hidden/
```

I had to code a small scraper in python for this. it took a few minutes to execute:
Got this

```txt
Hey, here is your flag : d5eec3ec36cf80dce44a896f961c1831a05526ec215693c8f2c39543497d4466
```