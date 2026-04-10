# Darkly CTF - Claude Project Guide

## Project Goal
Find and document **14 web security breaches** with manual exploitation (no sqlmap/automated tools).
Each breach must include: flag file + Resources folder with proof of concept.

## Key Principles
- **Manual exploitation only** - Understanding > automation
- **Document everything** - How, why, proof-of-concept code required
- **OWASP Top 10 focus** - SQLi, Auth, XSS, Access Control, Misconfiguration

## Environment
- VM: `http://localhost:8080` (QEMU, port-forwarded from 8080→80)
- Local mirror: `wget -mk http://localhost:8080` to analyze locally
- Tools allowed: Burp/ZAP, gobuster, curl, browser DevTools, Python scripts

## Progress Tracking
- Found: 2/14 flags
  - Member SQL Injection
  - Hidden CSS flag
- Next targets: redirects, uploads, LFI, XSS, hidden dirs

## Workflow
1. **Identify vulnerability** - Test attack surface
2. **Create breach folder** - `N-VulnerabilityName/`
3. **Document flag** - Save in `flag` file
4. **Build Resources** - Scripts, payloads, proof needed to reproduce
5. **Update README** - Add to project notes

## Quick Commands
```bash
# Local site copy
wget -mk http://localhost:8080

# Directory fuzzing (if gobuster available)
gobuster dir -u http://localhost:8080 -w /path/to/wordlist

# Test endpoints
curl http://localhost:8080/robots.txt
curl "http://localhost:8080/index.php?page=member&id=1"

# Check hash type
hashid "your_hash_string"
```

## Breach Folder Template
```
N-BbreachName/
├── flag          (the actual flag)
├── README.md     (how/why explanation)
└── Resources/    (proof: scripts, payloads, screenshots)
```

## Known Attack Surfaces
- `/whatever` & `/.hidden` (hidden dirs from robots.txt)
- `page=member` (SQL injection)
- `page=redirect&site=X` (open redirect)
- `page=upload` (file upload)
- `page=*` (LFI potential)
- Forms: signin, feedback, search (XSS)

## Notes
- NEVER use --no-verify or destructive git ops without confirming
- Keep memory files updated with patterns/findings
- Test locally with curl/bash before complex exploitation
