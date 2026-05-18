To convert a string like `"users"` into its hex representation (`0x7573657273`) directly from your terminal, you have a few excellent built-in options depending on your operating system (Linux/macOS).

Here are the easiest commands to use:

### 1. Using `xxd` (The Standard Way)

`xxd` is a dedicated tool for hex dumping. It is usually installed by default on most Linux distributions and macOS.

```bash
echo -n "users" | xxd -p

```

* **`-n`**: Tells `echo` not to include a newline character at the end (otherwise, the hex for the newline `0a` will be added to your result).
* **`-p`**: Tells `xxd` to output "plain" hex format without extra column formatting or memory addresses.
* **Output:** `7573657273` (Just prepend `0x` to use it in your SQLi payload: `0x7573657273`).

---

### 2. Using `od` (Octal Dump)

If `xxd` isn't installed, `od` is guaranteed to be there on any Unix-like system.

```bash
echo -n "users" | od -An -tx1 | tr -d ' '

```

* **`-An`**: Removes the address column from the output.
* **`-tx1`**: Formats the output as hex, 1 byte at a time.
* **`tr -d ' '`**: Strips out the spaces that `od` automatically puts between characters.
* **Output:** `7573657273`

---

### 3. Using `python` (Cross-Platform)

If you prefer a method that works identically on Linux, Mac, and Windows (as long as Python is installed):

```bash
python3 -c "print('users'.encode().hex())"

```

* **Output:** `7573657273`

---

### Bonus: Automate the `0x` prefix

If you want the terminal to automatically spit out the exact format MySQL expects (`0x...`), you can format the `xxd` output using `printf`:

```bash
printf "0x%s\n" $(echo -n "users" | xxd -p)

```

* **Output:** `0x7573657273`