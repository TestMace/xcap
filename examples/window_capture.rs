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

    dir::create_all("target/windows", true).unwrap();

    let mut i = 0;
    for window in windows {
        // 最小化的窗口不能截屏
        if window.is_minimized().unwrap() {
            continue;
        }

        let title = window.title().unwrap_or_else(|_| "<Unknown>".to_string());
        let (x, y, width, height) = (
            window.x().unwrap_or(0),
            window.y().unwrap_or(0),
            window.width().unwrap_or(0),
            window.height().unwrap_or(0),
        );

        println!(
            "Window: {:?} {:?} {:?}",
            title,
            (x, y, width, height),
            (
                window.is_minimized().unwrap_or(false),
                window.is_maximized().unwrap_or(false)
            )
        );

        // Skip windows with invalid dimensions
        if width <= 0 || height <= 0 {
            println!("  Skipping window with invalid dimensions: {}x{}", width, height);
            continue;
        }

        match window.capture_image() {
            Ok(image) => {
                if image.width() == 0 || image.height() == 0 {
                    println!("  Captured image has zero dimensions: {}x{}", image.width(), image.height());
                    continue;
                }
                
                match image.save(format!(
                    "target/windows/window-{}-{}.png",
                    i,
                    normalized(&title)
                )) {
                    Ok(_) => {
                        println!("  Saved: window-{}-{}.png ({}x{})", i, normalized(&title), image.width(), image.height());
                        i += 1;
                    }
                    Err(e) => {
                        println!("  Failed to save image for '{}': {}", title, e);
                    }
                }
            }
            Err(e) => {
                println!("  Failed to capture '{}': {}", title, e);
            }
        }
    }

    println!("运行耗时: {:?}", start.elapsed());
}
