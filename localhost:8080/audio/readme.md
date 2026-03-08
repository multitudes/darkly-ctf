To display the binary contents of an MP4 file in the terminal, you can use these commands:

1. Using hexdump:
```bash
hexdump -C yourfile.mp4 | less
```
- `-C` gives a canonical hex+ASCII view.
- `| less` lets you scroll.

2. Using xxd:
```bash
xxd yourfile.mp4 | less
```
- `xxd` is another hex viewer.

3. In vim:
```bash
vim yourfile.mp4
```
Then, inside vim, run:
```
:%!xxd
```
- This converts the file to hex view in vim.

To revert back in vim:
```
:%!xxd -r
```

Replace `yourfile.mp4` with your actual file name. These methods let you inspect the binary/hex contents easily.

To scroll down a page or search for a word in the terminal:

- If using less (e.g., after hexdump or xxd):
  - Scroll down: Press Space or Down Arrow.
  - Search for a word: Type /word and press Enter. Use n to jump to next match.

- If using vim:
  - Scroll down: Press Ctrl+F (page down) or Down Arrow.
  - Search for a word: Type /word and press Enter. Use n to jump to next match.

Replace "word" with what you want to find.