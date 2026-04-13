#!/bin/bash

# Test remaining pages for breaches 13 & 14
BASE="http://localhost:8081"

echo "=== Testing /index.php?page=search ==="
curl -s "$BASE/index.php?page=search&q=test" | head -20

echo -e "\n=== Testing /index.php?page=feedback ==="
curl -s "$BASE/index.php?page=feedback" | head -20

echo -e "\n=== Testing /index.php?page=contact ==="
curl -s "$BASE/index.php?page=contact" | head -20

echo -e "\n=== Testing /index.php?page=vote ==="
curl -s "$BASE/index.php?page=vote" | head -20

echo -e "\n=== Test reflected XSS on search ==="
curl -s "$BASE/index.php?page=search&q=%3Cscript%3Ealert(1)%3C/script%3E" | grep -i script

echo -e "\n=== Test reflected XSS on feedback ==="
curl -s "$BASE/index.php?page=feedback&comment=%3Cscript%3E" | grep -i script
