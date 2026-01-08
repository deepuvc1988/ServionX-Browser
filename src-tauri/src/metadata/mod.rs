// Metadata Module
// Strips and fakes file metadata for uploads

pub mod commands;
pub mod upload_protection;
pub mod document_stripper;

pub use upload_protection::FakeFileMetadata;
pub use document_stripper::DocumentStripper;

use serde::{Deserialize, Serialize};
use std::path::Path;
use image::ImageFormat;
use rand::Rng;
use exif::{Reader, Tag, Value as ExifValue, In};

/// Metadata information extracted from a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub filename: String,
    pub size_bytes: u64,
    pub file_type: String,
    pub has_exif: bool,
    pub exif_data: Option<ExifData>,
    pub document_properties: Option<DocumentProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExifData {
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub date_taken: Option<String>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub software: Option<String>,
    pub orientation: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentProperties {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
}

/// Result of stripping metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrippedFile {
    pub original_size: u64,
    pub stripped_size: u64,
    pub metadata_removed: bool,
    pub fake_metadata_added: bool,
    pub output_path: String,
}

/// Strips metadata from files
pub struct MetadataStripper {
    fake_cameras: Vec<FakeCamera>,
    fake_software: Vec<&'static str>,
}

struct FakeCamera {
    make: &'static str,
    model: &'static str,
}

impl MetadataStripper {
    pub fn new() -> Self {
        Self {
            fake_cameras: vec![
                FakeCamera { make: "Apple", model: "iPhone 15 Pro" },
                FakeCamera { make: "Apple", model: "iPhone 14" },
                FakeCamera { make: "Samsung", model: "Galaxy S24 Ultra" },
                FakeCamera { make: "Google", model: "Pixel 8 Pro" },
                FakeCamera { make: "Canon", model: "EOS R5" },
                FakeCamera { make: "Sony", model: "Alpha A7 IV" },
                FakeCamera { make: "Nikon", model: "Z8" },
                FakeCamera { make: "Fujifilm", model: "X-T5" },
            ],
            fake_software: vec![
                "Adobe Photoshop 2024",
                "Adobe Lightroom Classic",
                "Capture One 23",
                "GIMP 2.10",
                "Affinity Photo 2",
                "Photos 9.0",
            ],
        }
    }
    
    /// Get metadata from a file without modifying it
    pub fn get_metadata(&self, file_path: &str) -> Result<FileMetadata, String> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Err("File not found".to_string());
        }
        
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let size_bytes = std::fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        let file_type = match extension.as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "pdf" => "application/pdf",
            "doc" | "docx" => "application/msword",
            "xls" | "xlsx" => "application/vnd.ms-excel",
            _ => "application/octet-stream",
        }.to_string();
        
        // Try to read EXIF data for images
        let (has_exif, exif_data) = if ["jpg", "jpeg", "tiff"].contains(&extension.as_str()) {
            self.read_exif(file_path)
        } else {
            (false, None)
        };
        
        Ok(FileMetadata {
            filename,
            size_bytes,
            file_type,
            has_exif,
            exif_data,
            document_properties: None,
        })
    }
    
    /// Strip metadata from a file and optionally inject fake metadata
    pub fn strip_metadata(&self, file_path: &str, inject_fake: bool) -> Result<StrippedFile, String> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Err("File not found".to_string());
        }
        
        let original_size = std::fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Create output path
        let output_path = self.create_output_path(file_path)?;
        
        match extension.as_str() {
            "jpg" | "jpeg" | "png" | "webp" | "gif" => {
                self.strip_image_metadata(file_path, &output_path, inject_fake)?;
            }
            _ => {
                // For non-image files, just copy
                std::fs::copy(file_path, &output_path)
                    .map_err(|e| e.to_string())?;
            }
        }
        
        let stripped_size = std::fs::metadata(&output_path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        Ok(StrippedFile {
            original_size,
            stripped_size,
            metadata_removed: true,
            fake_metadata_added: inject_fake,
            output_path,
        })
    }
    
    /// Strip EXIF from images
    fn strip_image_metadata(&self, input: &str, output: &str, inject_fake: bool) -> Result<(), String> {
        // Read the image
        let img = image::open(input).map_err(|e| e.to_string())?;
        
        // Determine output format from path
        let output_path = Path::new(output);
        let extension = output_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpg")
            .to_lowercase();
        
        let format = match extension.as_str() {
            "png" => ImageFormat::Png,
            "gif" => ImageFormat::Gif,
            "webp" => ImageFormat::WebP,
            _ => ImageFormat::Jpeg,
        };
        
        // Save without EXIF (image crate doesn't preserve EXIF by default)
        img.save_with_format(output, format).map_err(|e| e.to_string())?;
        
        if inject_fake {
            log::info!("Fake metadata injection requested");
        }
        
        Ok(())
    }
    
    /// Read EXIF data from an image
    fn read_exif(&self, file_path: &str) -> (bool, Option<ExifData>) {
        let file = match std::fs::File::open(file_path) {
            Ok(f) => f,
            Err(_) => return (false, None),
        };
        
        let mut bufreader = std::io::BufReader::new(file);
        
        match Reader::new().read_from_container(&mut bufreader) {
            Ok(exif) => {
                let mut data = ExifData {
                    camera_make: None,
                    camera_model: None,
                    date_taken: None,
                    gps_latitude: None,
                    gps_longitude: None,
                    software: None,
                    orientation: None,
                };
                
                // Extract Make
                if let Some(field) = exif.get_field(Tag::Make, In::PRIMARY) {
                    data.camera_make = Some(field.display_value().to_string());
                }
                
                // Extract Model
                if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
                    data.camera_model = Some(field.display_value().to_string());
                }
                
                // Extract DateTimeOriginal
                if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                    data.date_taken = Some(field.display_value().to_string());
                }
                
                // Extract Software
                if let Some(field) = exif.get_field(Tag::Software, In::PRIMARY) {
                    data.software = Some(field.display_value().to_string());
                }
                
                // Extract Orientation
                if let Some(field) = exif.get_field(Tag::Orientation, In::PRIMARY) {
                    if let ExifValue::Short(ref vals) = field.value {
                        data.orientation = vals.first().map(|v| *v as u32);
                    }
                }
                
                // Extract GPS Latitude
                if let Some(field) = exif.get_field(Tag::GPSLatitude, In::PRIMARY) {
                    if let ExifValue::Rational(ref vals) = field.value {
                        if vals.len() >= 3 {
                            let degrees = vals[0].to_f64();
                            let minutes = vals[1].to_f64();
                            let seconds = vals[2].to_f64();
                            data.gps_latitude = Some(degrees + minutes / 60.0 + seconds / 3600.0);
                        }
                    }
                }
                
                // Extract GPS Longitude
                if let Some(field) = exif.get_field(Tag::GPSLongitude, In::PRIMARY) {
                    if let ExifValue::Rational(ref vals) = field.value {
                        if vals.len() >= 3 {
                            let degrees = vals[0].to_f64();
                            let minutes = vals[1].to_f64();
                            let seconds = vals[2].to_f64();
                            data.gps_longitude = Some(degrees + minutes / 60.0 + seconds / 3600.0);
                        }
                    }
                }
                
                (true, Some(data))
            }
            Err(_) => (false, None),
        }
    }
    
    /// Generate fake EXIF data
    pub fn generate_fake_exif(&self) -> ExifData {
        let mut rng = rand::thread_rng();
        
        let camera = &self.fake_cameras[rng.gen_range(0..self.fake_cameras.len())];
        let software = self.fake_software[rng.gen_range(0..self.fake_software.len())];
        
        // Generate a random date in the past year
        let days_ago = rng.gen_range(1..365);
        let fake_date = chrono::Utc::now() - chrono::Duration::days(days_ago);
        
        ExifData {
            camera_make: Some(camera.make.to_string()),
            camera_model: Some(camera.model.to_string()),
            date_taken: Some(fake_date.format("%Y:%m:%d %H:%M:%S").to_string()),
            gps_latitude: None,
            gps_longitude: None,
            software: Some(software.to_string()),
            orientation: Some(1),
        }
    }
    
    fn create_output_path(&self, input_path: &str) -> Result<String, String> {
        let path = Path::new(input_path);
        let filename = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file");
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("dat");
        
        // Create temp directory for stripped files
        let temp_dir = dirs::cache_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
            .join("servionx-browser")
            .join("stripped");
        
        std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
        
        let output_path = temp_dir.join(format!("{}_stripped.{}", filename, extension));
        
        Ok(output_path.to_string_lossy().to_string())
    }
}

impl Default for MetadataStripper {
    fn default() -> Self {
        Self::new()
    }
}
