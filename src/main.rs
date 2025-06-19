use anyhow::{Context, Result, bail};
use clap::Parser;
use qpdf::*;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple CLI tool to shuffle PDF pages for double-sided scanning", long_about = None)]
struct Args {
    /// Input PDF file path
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Check if input file exists
    if !args.input.exists() {
        bail!("Input file '{}' does not exist", args.input.display());
    }

    // Generate output filename
    let output_path = generate_output_path(&args.input)?;

    // Process the PDF
    shuffle_pdf(&args.input, &output_path)?;

    println!(
        "Successfully shuffled PDF: {} -> {}",
        args.input.display(),
        output_path.display()
    );

    Ok(())
}

fn generate_output_path(input_path: &Path) -> Result<PathBuf> {
    let stem = input_path
        .file_stem()
        .context("Invalid input file path")?
        .to_string_lossy();

    let extension = input_path
        .extension()
        .context("Input file has no extension")?;

    if extension != "pdf" {
        bail!(
            "Input file must be a PDF (has extension: {})",
            extension.to_string_lossy()
        );
    }

    let output_filename = format!("{}_shuff.pdf", stem);

    let mut output_path = input_path.to_path_buf();
    output_path.set_file_name(output_filename);

    Ok(output_path)
}

fn shuffle_pdf(input_path: &Path, output_path: &Path) -> Result<()> {
    // Open the input PDF
    let input_pdf = QPdf::read(input_path)?;

    // Get all pages and count them
    let all_pages = input_pdf.get_pages()?;
    let page_count = all_pages.len();

    // Check if page count is even
    if page_count % 2 != 0 {
        bail!(
            "PDF has an odd number of pages ({}). This tool only works with an even number of pages.",
            page_count
        );
    }

    if page_count == 0 {
        bail!("PDF has no pages");
    }

    let half = page_count / 2;

    // Create a new PDF for output
    let output_pdf = QPdf::empty();

    // Copy pages in the shuffled order
    for i in 0..half {
        // Add front page (from first half)
        let front_page = all_pages
            .get(i)
            .ok_or_else(|| anyhow::anyhow!("Failed to get page {}", i + 1))?;
        output_pdf.add_page(front_page, false)?;

        // Add back page (from second half)
        let back_page = all_pages
            .get(half + i)
            .ok_or_else(|| anyhow::anyhow!("Failed to get page {}", half + i + 1))?;
        output_pdf.add_page(back_page, false)?;
    }

    // Write the output PDF to memory and then to file
    let pdf_data = output_pdf.writer().write_to_memory()?;
    std::fs::write(output_path, pdf_data)?;

    Ok(())
}
