use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use ::image::io::Reader as ImageReader;
use ::image::GenericImageView;

#[tauri::command]
fn convert_images_to_pdf(image_paths: Vec<String>, output_path: String) -> Result<String, String> {
    if image_paths.is_empty() {
        return Err("No images selected".to_string());
    }

    // A4 size in mm
    let page_w = Mm(210.0);
    let page_h = Mm(297.0);

    // Create document with first page (A4)
    #[allow(unused_mut)]
    let (mut doc, page1, layer1) = PdfDocument::new(
        "Image to PDF",
        page_w,
        page_h,
        "Layer 1",
    );

    let mut is_first = true;

    for path_str in &image_paths {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err(format!("File not found: {}", path_str));
        }

        let dyn_img = ImageReader::open(path)
            .map_err(|e| format!("Failed to open image {}: {}", path_str, e))?
            .decode()
            .map_err(|e| format!("Failed to decode image {}: {}", path_str, e))?;

        let (img_w, img_h) = dyn_img.dimensions();
        let pdf_image = Image::from_dynamic_image(&dyn_img);

        let (page, layer) = if is_first {
            is_first = false;
            (page1, layer1)
        } else {
            doc.add_page(page_w, page_h, "Layer 1")
        };

        let current_layer = doc.get_page(page).get_layer(layer);

        // Fit image to the full page while preserving aspect ratio (allow upscaling)
        let px_to_mm = 25.4 / 300.0;
        let img_w_mm = img_w as f32 * px_to_mm;
        let img_h_mm = img_h as f32 * px_to_mm;

        let scale_w = page_w.0 / img_w_mm;
        let scale_h = page_h.0 / img_h_mm;
        let scale = scale_w.min(scale_h);

        let scaled_w = img_w_mm * scale;
        let scaled_h = img_h_mm * scale;

        // Center on page
        let x = (page_w.0 - scaled_w) / 2.0;
        let y = (page_h.0 - scaled_h) / 2.0;

        pdf_image.add_to_layer(
            current_layer,
            ImageTransform {
                translate_x: Some(Mm(x)),
                translate_y: Some(Mm(y)),
                scale_x: Some(scale),
                scale_y: Some(scale),
                dpi: Some(300.0),
                ..Default::default()
            },
        );
    }

    // Save the PDF
    let output = Path::new(&output_path);
    let file = File::create(output).map_err(|e| format!("Cannot create output file: {}", e))?;
    let mut buf_writer = BufWriter::new(file);
    doc.save(&mut buf_writer)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(format!("PDF başarıyla kaydedildi: {}", output_path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![convert_images_to_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
