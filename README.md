# pdfshuff

A simple tool to fix PDFs that were scanned twice on a single-sided scanner.

## What it does

When you scan a double-sided document on a single-sided scanner, you typically:

1. Scan all the front pages (1, 3, 5, 7...)
2. Flip the stack and scan all the back pages (8, 6, 4, 2...)

This results in a PDF with pages in the wrong order: [1, 3, 5, 7, 8, 6, 4, 2]

pdfshuff fixes this by interleaving the pages correctly: [1, 2, 3, 4, 5, 6, 7, 8]

## Usage

Just drag and drop your PDF files onto the window. The tool will create a new file with `_shuff` suffix.

## Build

```bash
cargo build --release
```

## License

Licensed under either of Apache License 2.0 or MIT license at your option.
