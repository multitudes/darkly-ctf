# the survey page

Examining the html page of the survey I did not see anything suspicious. I used burp to intercept the post request after changing some data in the page.

I saw that it sends the post request as

```txt
sujet=2&valeur=2
```

so i changed it to:

```txt
sujet=42&valeur=42
```

and got the flag displayed.

## using curl

```bash
curl -X POST http://localhost:8081/?page=survey \
     -d "sujet=42&valeur=42"
```
