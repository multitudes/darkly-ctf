
# the .hidden directory

found in the robots.txt file

```text
Disallow: /.hidden  
```

so i went there and found a long ist of recursively created random links which pages of more random links.

```txt
http://localhost:8080/.hidden/
```

Looking like this:
```txt
Index of /.hidden/

../
amcbevgondgcrloowluziypjdh/                        29-Jun-2021 18:15                   -
bnqupesbgvhbcwqhcuynjolwkm/                        29-Jun-2021 18:15                   -
ceicqljdddshxvnvdqzzjgddht/                        29-Jun-2021 18:15                   -
doxelitrqvhegnhlhrkdgfizgj/                        29-Jun-2021 18:15                   -
eipmnwhetmpbhiuesykfhxmyhr/                        29-Jun-2021 18:15                   -
ffpbexkomzbigheuwhbhbfzzrg/                        29-Jun-2021 18:15                   -
ghouhyooppsmaizbmjhtncsvfz/                        29-Jun-2021 18:15                   -
hwlayeghtcotqdigxuigvjufqn/                        29-Jun-2021 18:15                   -
isufpcgmngmrotmrjfjonpmkxu/                        29-Jun-2021 18:15                   -
jfiombdhvlwxrkmawgoruhbarp/                        29-Jun-2021 18:15                   -
kpibbgxjqnvrrcpczovjbvijmz/                        29-Jun-2021 18:15                   -
ldtafmsxvvydthtgflzhadiozs/                        29-Jun-2021 18:15                   -
mrucagbgcenowkjrlmmugvztuh/                        29-Jun-2021 18:15                   -
ntyrhxjbtndcpjevzurlekwsxt/                        29-Jun-2021 18:15                   -
oasstobmotwnezhscjjopenjxy/                        29-Jun-2021 18:15                   -
ppjxigqiakcrmqfhotnncfqnqg/                        29-Jun-2021 18:15                   -
qcwtnvtdfslnkvqvzhjsmsghfw/                        29-Jun-2021 18:15                   -
rlnoyduccpqxkvcfiqpdikfpvx/                        29-Jun-2021 18:15                   -
sdnfntbyirzllbpctnnoruyjjc/                        29-Jun-2021 18:15                   -
trwjgrgmfnzarxiiwvwalyvanm/                        29-Jun-2021 18:15                   -
urhkbrmupxbgdnntopklxskvom/                        29-Jun-2021 18:15                   -
viphietzoechsxwqacvpsodhaq/                        29-Jun-2021 18:15                   -
whtccjokayshttvxycsvykxcfm/                        29-Jun-2021 18:15                   -
xuwrcwjjrmndczfcrmwmhvkjnh/                        29-Jun-2021 18:15                   -
yjxemfsgdlkbvvtjiylhdoaqkn/                        29-Jun-2021 18:15                   -
zzfzjvjsupgzinctxeqtzzdzll/                        29-Jun-2021 18:15                   -
README       
```

I had to code a small scraper in python for this. it took a few minutes to execute. Run with:

```bash
uv run --with requests --with beautifulsoup4 scraper.py
```

Got this:

```txt
Hey, here is your flag : d5eec3ec36cf80dce44a896f961c1831a05526ec215693c8f2c39543497d4466
```

