use image::io::Reader as ImageReader;
use std::fs;
use std::path::Path;

fn check_image_dimensions(file_path: &Path) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let img = ImageReader::open(file_path)?.decode()?;
    Ok((img.width(), img.height()))
}

fn main() {
    println!("=== Verifying Saved Image Dimensions ===\n");
    
    // Check matching examples
    println!("📁 Matching Examples:");
    if let Ok(entries) = fs::read_dir("target/dimension_test/matching") {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if extension == "png" {
                    match check_image_dimensions(&entry.path()) {
                        Ok((width, height)) => {
                            println!("  ✅ {} -> {}x{}", 
                                entry.file_name().to_string_lossy(), width, height);
                        }
                        Err(e) => {
                            println!("  ❌ {} -> Error: {}", 
                                entry.file_name().to_string_lossy(), e);
                        }
                    }
                }
            }
        }
    }
    
    println!("\n📁 Mismatched Examples:");
    if let Ok(entries) = fs::read_dir("target/dimension_test/mismatched") {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if extension == "png" {
                    match check_image_dimensions(&entry.path()) {
                        Ok((width, height)) => {
                            println!("  📸 {} -> {}x{}", 
                                entry.file_name().to_string_lossy(), width, height);
                        }
                        Err(e) => {
                            println!("  ❌ {} -> Error: {}", 
                                entry.file_name().to_string_lossy(), e);
                        }
                    }
                }
            }
        }
    }
    
    println!("\n✅ Verification complete!");
} 