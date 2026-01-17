use arboard::Clipboard;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::{DynamicImage, ImageFormat};
use log::{debug, error, info};
use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ClipboardManager {
    clipboard: Arc<Mutex<Clipboard>>,
    last_image_hash: Arc<Mutex<Option<String>>>,
}

impl ClipboardManager {
    pub fn new() -> Result<Self, String> {
        let clipboard = Clipboard::new()
            .map_err(|e| format!("Failed to create clipboard: {}", e))?;

        Ok(Self {
            clipboard: Arc::new(Mutex::new(clipboard)),
            last_image_hash: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn get_image(&self) -> Result<Option<String>, String> {
        let mut clipboard = self.clipboard.lock().await;
        
        match clipboard.get_image() {
            Ok(img_data) => {
                // Check minimum size (64x64)
                if img_data.width < 64 || img_data.height < 64 {
                    debug!("Image too small: {}x{}", img_data.width, img_data.height);
                    return Ok(None);
                }

                // Convert to PNG and base64
                let rgba_data: Vec<u8> = img_data.bytes.to_vec();
                let img = DynamicImage::ImageRgba8(
                    image::RgbaImage::from_raw(
                        img_data.width as u32,
                        img_data.height as u32,
                        rgba_data,
                    )
                    .ok_or_else(|| "Failed to create image from clipboard data".to_string())?,
                );

                let mut png_data = Vec::new();
                let mut cursor = Cursor::new(&mut png_data);
                img.write_to(&mut cursor, ImageFormat::Png)
                    .map_err(|e| format!("Failed to encode image as PNG: {}", e))?;

                let base64_str = STANDARD.encode(&png_data);
                
                // Calculate hash for deduplication
                let hash = format!("{:x}", md5::compute(&png_data));
                
                let mut last_hash = self.last_image_hash.lock().await;
                if last_hash.as_ref() == Some(&hash) {
                    debug!("Same image detected, skipping");
                    return Ok(None);
                }
                
                *last_hash = Some(hash);
                info!("New image detected: {}x{}", img_data.width, img_data.height);
                
                Ok(Some(base64_str))
            }
            Err(arboard::Error::ContentNotAvailable) => {
                debug!("No image in clipboard");
                Ok(None)
            }
            Err(e) => {
                error!("Failed to get clipboard image: {}", e);
                Err(format!("Failed to get clipboard image: {}", e))
            }
        }
    }

    pub async fn set_image(&self, base64_data: &str) -> Result<(), String> {
        let png_data = STANDARD
            .decode(base64_data)
            .map_err(|e| format!("Failed to decode base64: {}", e))?;

        let img = image::load_from_memory(&png_data)
            .map_err(|e| format!("Failed to load image: {}", e))?;

        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();

        let img_data = arboard::ImageData {
            width: width as usize,
            height: height as usize,
            bytes: rgba_img.into_raw().into(),
        };

        let mut clipboard = self.clipboard.lock().await;
        clipboard
            .set_image(img_data)
            .map_err(|e| format!("Failed to set clipboard image: {}", e))?;

        info!("Image set to clipboard");
        Ok(())
    }
}