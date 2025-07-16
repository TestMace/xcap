use fs_extra::dir;
use std::time::Instant;
use xcap::Window;

fn normalized(filename: &str) -> String {
    filename.chars()
        .map(|c| match c {
            '|' | '\\' | ':' | '/' | '*' | '?' | '"' | '<' | '>' | '(' | ')' => '_',
            c if c.is_control() => '_',
            c if c as u32 > 127 => '_', // Replace non-ASCII characters
            c => c,
        })
        .collect()
}

fn main() {
    let start = Instant::now();
    let windows = Window::all().unwrap();

    // Create directories for test results
    dir::create_all("target/dimension_test/matching", true).unwrap();
    dir::create_all("target/dimension_test/mismatched", true).unwrap();

    let mut total_windows = 0;
    let mut captured_windows = 0;
    let mut matching_dimensions = 0;
    let mut mismatched_dimensions = 0;
    let mut matching_examples_saved = 0;
    let mut mismatch_examples_saved = 0;

    println!("=== Window Dimension Test ===");
    println!("Testing that window.width() and window.height() match captured image dimensions\n");

    for window in windows {
        total_windows += 1;

        // Skip minimized windows as they can't be captured properly
        if window.is_minimized().unwrap_or(true) {
            continue;
        }

        let title = window.title().unwrap_or_else(|_| "<Unknown>".to_string());
        let app_name = window.app_name().unwrap_or_else(|_| "<Unknown>".to_string());
        
        let (window_width, window_height) = match (window.width(), window.height()) {
            (Ok(w), Ok(h)) => (w, h),
            _ => {
                println!("⚠️  Failed to get dimensions for: {}", title);
                continue;
            }
        };

        // Skip windows with invalid dimensions
        if window_width == 0 || window_height == 0 {
            continue;
        }

        match window.capture_image() {
            Ok(image) => {
                captured_windows += 1;
                let image_width = image.width();
                let image_height = image.height();

                let dimensions_match = window_width == image_width && window_height == image_height;
                
                if dimensions_match {
                    matching_dimensions += 1;
                    println!("✅ MATCH: {} ({}x{}) - App: {}", 
                        title, window_width, window_height, app_name);
                    
                    // Save a few examples of matching cases
                    if matching_examples_saved < 3 {
                        let filename = format!(
                            "target/dimension_test/matching/match-{}-{}.png",
                            matching_examples_saved,
                            normalized(&title)
                        );
                        if image.save(&filename).is_ok() {
                            matching_examples_saved += 1;
                            println!("   💾 Saved example: {}", filename);
                        }
                    }
                } else {
                    mismatched_dimensions += 1;
                    println!("❌ MISMATCH: {}", title);
                    println!("   App: {}", app_name);
                    println!("   Window dimensions: {}x{}", window_width, window_height);
                    println!("   Image dimensions:  {}x{}", image_width, image_height);
                    println!("   Difference: {}x{}", 
                        image_width as i32 - window_width as i32,
                        image_height as i32 - window_height as i32);
                    
                    // Save all mismatch examples for analysis
                    let filename = format!(
                        "target/dimension_test/mismatched/mismatch-{}-{}.png",
                        mismatch_examples_saved,
                        normalized(&title)
                    );
                    if image.save(&filename).is_ok() {
                        mismatch_examples_saved += 1;
                        println!("   💾 Saved mismatch example: {}", filename);
                    }
                    println!();
                }
            }
            Err(e) => {
                println!("⚠️  Failed to capture '{}': {}", title, e);
            }
        }
    }

    println!("\n=== Test Results ===");
    println!("Total windows found: {}", total_windows);
    println!("Successfully captured: {}", captured_windows);
    println!("Matching dimensions: {} ({}%)", 
        matching_dimensions, 
        if captured_windows > 0 { matching_dimensions * 100 / captured_windows } else { 0 });
    println!("Mismatched dimensions: {} ({}%)", 
        mismatched_dimensions,
        if captured_windows > 0 { mismatched_dimensions * 100 / captured_windows } else { 0 });
    
    println!("\n=== Examples Saved ===");
    println!("Matching examples: {} in target/dimension_test/matching/", matching_examples_saved);
    println!("Mismatch examples: {} in target/dimension_test/mismatched/", mismatch_examples_saved);
    
    if mismatched_dimensions == 0 {
        println!("\n🎉 SUCCESS: All captured windows have matching dimensions!");
    } else {
        println!("\n⚠️  {} windows have mismatched dimensions. Check saved examples for analysis.", mismatched_dimensions);
    }
    
    println!("\nTest completed in: {:?}", start.elapsed());
} 