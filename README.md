# darkly-ctf

A ctf 42 project for web security. Part of the outer core curriculum.

The goal is to find **14 different breaches** on a provided virtual machine, each rewarding you with a "flag". Per the subject, you must be able to explain the "how" and "why" of every exploit, making understanding the underlying concept more important than just getting the flag.

To start the vm contained in the iso file on the linus system on `localhost:8081`:  
```
qemu-system-x86_64 -m 1G -drive file=Darkly_i386.iso,format=raw,media=cdrom -boot d -net nic -net user,hostfwd=tcp::8081-:80
```

## links
The freecodecamp youtube course it is a good starting point. It is old but so is the 42 project:  
https://www.freecodecamp.org/news/web-app-penetration-testing-full-course/  
https://www.youtube.com/watch?v=2_lswM1S264  
They mention other tools like Burp Suite and ZAP. It is a good to learn to use those tools! They allowed to see clearly the requests and responses and make the discovery easier than just use a curl from your command line!  
https://portswigger.net/burp/communitydownload  
https://www.zaproxy.org/download/  

To practice:  
Damn Vulnerable Web Application:  
https://github.com/digininja/DVWA   
https://en.wikipedia.org/wiki/Damn_Vulnerable_Web_Application    
https://docs.metasploit.com/docs/using-metasploit/getting-started/  nightly-installers.html  

