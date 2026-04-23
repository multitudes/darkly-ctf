## Apple silicon

I used QEMU emulation because the target ISO is built for i386 (Intel 32-bit) while the hardware is ARM64 (Apple Silicon).

Make sure you have that emulator installed:

```bash
brew install qemu
which qemu-system-x86_64
```

You can run using this command:

``` bash
qemu-system-x86_64 -cdrom Darkly_i386.iso -m 1024 -cpu qemu64 -netdev user,id=net0,hostfwd=tcp::8080-:80 -device e1000,netdev=net0
```

Here is a breakdown of what each part of that command is doing:

Command Breakdown:
qemu-system-x86_64: This starts the emulator for a 64-bit Intel/AMD architecture. Even though the ISO is i386 (32-bit), this version is backward compatible and generally more stable on M1 Macs.
-cdrom Darkly_i386.iso: This tells the virtual machine to treat your Darkly_i386.iso file as a physical CD inserted into the drive.
-m 1024: This allocates 1024 MB (1 GB) of RAM to the virtual machine. This is plenty for a simple security audit project like Darkly.
-cpu qemu64: This defines the model of the CPU the VM "sees." Using a standard qemu64 profile ensures the virtual environment stays stable while emulating Intel instructions on your M1 chip.
-netdev user,id=net0,hostfwd=tcp::8080-:80. It sets up Port Forwarding:
It maps port 8080 on the Mac to port 80 (the web server) inside the VM.
To access the site: You won't use the IP shown in the VM prompt. Instead, you will open your browser and go to http://localhost:8080.
-device e1000,netdev=net0: This specifies the virtual network card (Intel E1000) for the VM to use so it can actually communicate with your Mac.
