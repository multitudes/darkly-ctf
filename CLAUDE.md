# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Darkly CTF - Claude Project Guide

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

---

## Common Development Commands

```bash
# Start the vulnerable VM (requires QEMU installed)
qemu-system-x86_64 -cdrom Darkly_i386.iso -m 1024 -cpu qemu64 \
  -netdev user,id=net0,hostfwd=tcp::8080-:80 -device e1000,netdev=net0

# Stop the VM (Ctrl+C in the terminal where it runs)
# No dedicated shutdown command; terminate the process.

# Mirror the target website for offline analysis
wget -mk http://localhost:8080 -P site-mirror

# Run a Python helper script (example: recursive hidden directory scraper)
python3 6-recursive-hidden-directory/Resources/scraper.py

# Perform directory fuzzing with gobuster
# Replace /path/to/wordlist with your chosen wordlist file.
gobuster dir -u http://localhost:8080 -w /path/to/wordlist -t 50

# Send a crafted request with curl (e.g., test open redirect)
curl "http://localhost:8080/index.php?page=redirect&site=https://example.com"
```

## High-Level Architecture & Structure

- **Root Directory**: Contains the VM ISO (`Darkly_i386.iso`), top‑level `README.md`, and this `CLAUDE.md` guide.
- **Breach Folders (`N‑<Name>/`)**: One folder per discovered vulnerability, numbered in discovery order. Each folder follows the template:
  - `flag` – file containing the raw flag string.
  - `README.md` – description of the vulnerability, exploitation steps, and lesson learned.
  - `Resources/` – scripts, payloads, screenshots, and any auxiliary files needed to reproduce the exploit.
- **Shared Resources**: Some exploits ship reusable scripts (e.g., the hidden‑directory scraper under `6-recursive-hidden-directory/Resources/`). These can be executed against the live VM or the mirrored site.
- **Static Web Content**: The original web application files are not stored in the repo; they are served by the VM at runtime. Use `wget`/`curl` to fetch pages for analysis.
- **Git Tracking**: All flag files and documentation are version‑controlled. No source code of the vulnerable app lives here, so typical build/lint steps do not apply.
- **Testing Strategy**: Verify each exploit against the running VM or its local mirror before committing the flag and resources.

---

*End of CLAUDE.md guidance.*
