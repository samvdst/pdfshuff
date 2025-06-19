# pdfshuff

A simple tool to fix PDFs that were scanned twice on a single-sided scanner.

## What it does

When you scan a double-sided document on a single-sided scanner, you typically:

1. Scan all the front pages (1, 3, 5, 7...)
2. Flip the stack and scan all the back pages (8, 6, 4, 2...)

This results in a PDF with pages in the wrong order: [1, 3, 5, 7, 8, 6, 4, 2]

pdfshuff fixes this by interleaving the pages correctly: [1, 2, 3, 4, 5, 6, 7, 8]

**Important:** Make sure "skip empty pages" is disabled on your scanner. If enabled, the page count might not match and the interleaving will be incorrect.

## Usage

Just drag and drop your PDF files onto the window. The tool will create a new file with `_shuff` suffix.

## Build

```bash
cargo build --release
```

## Disclaimer

This is a personal tool I built to solve my own scanning workflow problem. While I'm happy to share it and hope others find it useful, please note that:

- This is provided as-is, without warranties
- I haven't tested all platform binaries
- I am not able to address feature requests or provide support

That said, if it helps make your scanning workflow easier, that's awesome! Feel free to fork and adapt it to your needs.

## License

Licensed under either of Apache License 2.0 or MIT license at your option.
