#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release builds

use anyhow::{Context, Result, bail};
use eframe::egui;
use qpdf::*;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};
use std::thread;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("PDF Shuffler"),
        ..Default::default()
    };

    eframe::run_native(
        "PDF Shuffler",
        options,
        Box::new(|_cc| Ok(Box::new(PdfShufflerApp::default()))),
    )
}

#[derive(Default)]
struct PdfShufflerApp {
    status: String,
    status_receiver: Option<Receiver<String>>,
    is_processing: bool,
}

impl eframe::App for PdfShufflerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for status updates from background thread
        if let Some(receiver) = &self.status_receiver {
            if let Ok(status) = receiver.try_recv() {
                self.status = status;
                self.is_processing = false;
                self.status_receiver = None;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);

                ui.heading("PDF Shuffler");
                ui.add_space(20.0);

                ui.label("Drag and drop a PDF file here");
                ui.add_space(10.0);
                ui.label("to shuffle pages for double-sided scanning");

                ui.add_space(40.0);

                // Show status
                if !self.status.is_empty() {
                    if self.is_processing {
                        ui.spinner();
                    }
                    ui.label(&self.status);
                }
            });
        });

        // Handle file drops
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() && !self.is_processing {
                let dropped_file = &i.raw.dropped_files[0];

                if let Some(path) = &dropped_file.path {
                    self.process_file(path.clone());
                }
            }
        });

        // Enable file dropping
        preview_files_being_dropped(ctx);
    }
}

impl PdfShufflerApp {
    fn process_file(&mut self, path: PathBuf) {
        let (sender, receiver) = channel();
        self.status_receiver = Some(receiver);
        self.is_processing = true;
        self.status = "Processing...".to_string();

        thread::spawn(move || {
            let result = process_pdf(&path);
            match result {
                Ok(output_path) => {
                    let _ = sender.send(format!(
                        "✓ Success! Saved to: {}",
                        output_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                    ));
                }
                Err(e) => {
                    let _ = sender.send(format!("✗ Error: {}", e));
                }
            }
        });
    }
}

fn process_pdf(input_path: &Path) -> Result<PathBuf> {
    // Check if it's a PDF
    if input_path.extension().and_then(|e| e.to_str()) != Some("pdf") {
        bail!("File must be a PDF");
    }

    // Generate output filename
    let output_path = generate_output_path(input_path)?;

    // Process the PDF
    shuffle_pdf(input_path, &output_path)?;

    Ok(output_path)
}

fn generate_output_path(input_path: &Path) -> Result<PathBuf> {
    let stem = input_path
        .file_stem()
        .context("Invalid input file path")?
        .to_string_lossy();

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

        // Add back page (from second half, reversed)
        let back_page = all_pages
            .get(page_count - 1 - i)
            .ok_or_else(|| anyhow::anyhow!("Failed to get page {}", page_count - i))?;
        output_pdf.add_page(back_page, false)?;
    }

    // Write the output PDF to memory and then to file
    let pdf_data = output_pdf.writer().write_to_memory()?;
    std::fs::write(output_path, pdf_data)?;

    Ok(())
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let painter = ctx.layer_painter(LayerId::new(
            Order::Foreground,
            Id::new("file_drop_overlay"),
        ));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            "Drop to shuffle PDF",
            FontId::proportional(24.0),
            Color32::WHITE,
        );
    }
}
