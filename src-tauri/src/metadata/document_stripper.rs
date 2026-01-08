//! Document Metadata Stripping
//! Strips and injects fake metadata from PDF, Office (DOCX, XLSX, PPTX), and other document formats

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::io::{Read, Write, Seek};
use regex::Regex;

/// Supported document types for metadata stripping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    Pdf,
    Docx,
    Xlsx,
    Pptx,
    Odt,  // OpenDocument Text
    Ods,  // OpenDocument Spreadsheet
    Unknown,
}

/// Document metadata that can be stripped/faked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub keywords: Option<String>,
    pub company: Option<String>,
    pub manager: Option<String>,
    pub revision: Option<String>,
    pub total_edit_time: Option<String>,
}

/// Fake metadata profiles for injection
#[derive(Debug, Clone)]
pub struct FakeDocumentMetadata {
    pub author: String,
    pub creator: String,
    pub producer: String,
    pub company: String,
}

impl Default for FakeDocumentMetadata {
    fn default() -> Self {
        Self {
            author: "User".to_string(),
            creator: "Document Editor".to_string(),
            producer: "ServionX Protected".to_string(),
            company: String::new(),
        }
    }
}

/// Document metadata stripper - FULL implementation
pub struct DocumentStripper {
    fake_metadata: FakeDocumentMetadata,
}

impl Default for DocumentStripper {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentStripper {
    pub fn new() -> Self {
        Self {
            fake_metadata: FakeDocumentMetadata::default(),
        }
    }
    
    /// Create with custom fake metadata
    pub fn with_fake_metadata(fake_metadata: FakeDocumentMetadata) -> Self {
        Self { fake_metadata }
    }
    
    /// Detect document type from file extension
    pub fn detect_type(&self, file_path: &str) -> DocumentType {
        let path = Path::new(file_path);
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            "pdf" => DocumentType::Pdf,
            "docx" | "doc" => DocumentType::Docx,
            "xlsx" | "xls" => DocumentType::Xlsx,
            "pptx" | "ppt" => DocumentType::Pptx,
            "odt" => DocumentType::Odt,
            "ods" => DocumentType::Ods,
            _ => DocumentType::Unknown,
        }
    }
    
    /// Strip metadata from a PDF file
    pub fn strip_pdf_metadata(&self, input_path: &str, output_path: &str) -> Result<StrippingResult, String> {
        let mut file = std::fs::File::open(input_path).map_err(|e| e.to_string())?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).map_err(|e| e.to_string())?;
        
        let original_size = content.len();
        let content_str = String::from_utf8_lossy(&content).to_string();
        
        // Track what was found and removed
        let mut removed_fields = Vec::new();
        let mut modified_content = content_str.clone();
        
        // PDF metadata patterns to strip/replace
        let patterns = [
            (r"/Author\s*\([^)]*\)", "/Author (User)", "Author"),
            (r"/Author\s*<[^>]*>", "/Author <55736572>", "Author"),
            (r"/Creator\s*\([^)]*\)", "/Creator (Document Editor)", "Creator"),
            (r"/Creator\s*<[^>]*>", "/Creator <446F63756D656E74204564697 46F72>", "Creator"),
            (r"/Producer\s*\([^)]*\)", "/Producer (ServionX Protected)", "Producer"),
            (r"/Producer\s*<[^>]*>", "/Producer <53657276696F6E582050726F746563746564>", "Producer"),
            (r"/Title\s*\([^)]*\)", "/Title ()", "Title"),
            (r"/Title\s*<[^>]*>", "/Title <>", "Title"),
            (r"/Subject\s*\([^)]*\)", "/Subject ()", "Subject"),
            (r"/Subject\s*<[^>]*>", "/Subject <>", "Subject"),
            (r"/Keywords\s*\([^)]*\)", "/Keywords ()", "Keywords"),
            (r"/Keywords\s*<[^>]*>", "/Keywords <>", "Keywords"),
            (r"/Company\s*\([^)]*\)", "/Company ()", "Company"),
            (r"/Manager\s*\([^)]*\)", "/Manager ()", "Manager"),
        ];
        
        for (pattern, replacement, field_name) in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&modified_content) {
                    modified_content = re.replace_all(&modified_content, replacement).to_string();
                    removed_fields.push(field_name.to_string());
                }
            }
        }
        
        // Write the modified PDF
        let output_bytes = modified_content.as_bytes();
        std::fs::write(output_path, output_bytes).map_err(|e| e.to_string())?;
        
        let output_size = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0) as usize;
        
        log::info!("PDF metadata stripped: {} fields modified", removed_fields.len());
        
        Ok(StrippingResult {
            document_type: DocumentType::Pdf,
            original_size,
            output_size,
            fields_removed: removed_fields,
            fake_metadata_injected: true,
            output_path: output_path.to_string(),
        })
    }
    
    /// Strip metadata from Office documents (DOCX, XLSX, PPTX)
    /// These are ZIP files containing XML metadata in docProps/
    pub fn strip_office_metadata(&self, input_path: &str, output_path: &str) -> Result<StrippingResult, String> {
        let doc_type = self.detect_type(input_path);
        
        // Open the ZIP file
        let file = std::fs::File::open(input_path).map_err(|e| e.to_string())?;
        let original_size = file.metadata().map(|m| m.len()).unwrap_or(0) as usize;
        
        let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        // Create output file
        let output_file = std::fs::File::create(output_path).map_err(|e| e.to_string())?;
        let mut output_archive = zip::ZipWriter::new(output_file);
        
        let mut removed_fields = Vec::new();
        
        // Process each file in the ZIP
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
            let entry_name = entry.name().to_string();
            
            // Get file options
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(entry.compression());
            
            if entry_name == "docProps/core.xml" {
                // Read and strip core.xml (Dublin Core metadata)
                let mut content = String::new();
                entry.read_to_string(&mut content).map_err(|e| e.to_string())?;
                
                let (stripped, fields) = self.strip_core_xml(&content);
                removed_fields.extend(fields);
                
                output_archive.start_file(&entry_name, options).map_err(|e| e.to_string())?;
                output_archive.write_all(stripped.as_bytes()).map_err(|e| e.to_string())?;
                
            } else if entry_name == "docProps/app.xml" {
                // Read and strip app.xml (Application metadata)
                let mut content = String::new();
                entry.read_to_string(&mut content).map_err(|e| e.to_string())?;
                
                let (stripped, fields) = self.strip_app_xml(&content);
                removed_fields.extend(fields);
                
                output_archive.start_file(&entry_name, options).map_err(|e| e.to_string())?;
                output_archive.write_all(stripped.as_bytes()).map_err(|e| e.to_string())?;
                
            } else if entry_name == "docProps/custom.xml" {
                // Strip custom properties entirely (replace with empty)
                let empty_custom = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/custom-properties"></Properties>"#;
                
                removed_fields.push("CustomProperties".to_string());
                
                output_archive.start_file(&entry_name, options).map_err(|e| e.to_string())?;
                output_archive.write_all(empty_custom.as_bytes()).map_err(|e| e.to_string())?;
                
            } else {
                // Copy other files unchanged
                let mut buffer = Vec::new();
                entry.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                
                if entry.is_dir() {
                    output_archive.add_directory(&entry_name, options).map_err(|e| e.to_string())?;
                } else {
                    output_archive.start_file(&entry_name, options).map_err(|e| e.to_string())?;
                    output_archive.write_all(&buffer).map_err(|e| e.to_string())?;
                }
            }
        }
        
        output_archive.finish().map_err(|e| e.to_string())?;
        
        let output_size = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0) as usize;
        
        log::info!("Office document metadata stripped: {} fields modified", removed_fields.len());
        
        Ok(StrippingResult {
            document_type: doc_type,
            original_size,
            output_size,
            fields_removed: removed_fields,
            fake_metadata_injected: true,
            output_path: output_path.to_string(),
        })
    }
    
    /// Strip metadata from core.xml (Dublin Core metadata)
    fn strip_core_xml(&self, content: &str) -> (String, Vec<String>) {
        let mut result = content.to_string();
        let mut removed = Vec::new();
        
        // Dublin Core fields to strip/replace
        let patterns = [
            (r"<dc:creator>[^<]*</dc:creator>", format!("<dc:creator>{}</dc:creator>", self.fake_metadata.author), "dc:creator"),
            (r"<cp:lastModifiedBy>[^<]*</cp:lastModifiedBy>", format!("<cp:lastModifiedBy>{}</cp:lastModifiedBy>", self.fake_metadata.author), "cp:lastModifiedBy"),
            (r"<dc:title>[^<]*</dc:title>", "<dc:title></dc:title>".to_string(), "dc:title"),
            (r"<dc:subject>[^<]*</dc:subject>", "<dc:subject></dc:subject>".to_string(), "dc:subject"),
            (r"<dc:description>[^<]*</dc:description>", "<dc:description></dc:description>".to_string(), "dc:description"),
            (r"<cp:keywords>[^<]*</cp:keywords>", "<cp:keywords></cp:keywords>".to_string(), "cp:keywords"),
            (r"<cp:category>[^<]*</cp:category>", "<cp:category></cp:category>".to_string(), "cp:category"),
            (r"<cp:revision>[^<]*</cp:revision>", "<cp:revision>1</cp:revision>".to_string(), "cp:revision"),
        ];
        
        for (pattern, replacement, field_name) in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&result) {
                    result = re.replace_all(&result, replacement.as_str()).to_string();
                    removed.push(field_name.to_string());
                }
            }
        }
        
        (result, removed)
    }
    
    /// Strip metadata from app.xml (Application metadata)
    fn strip_app_xml(&self, content: &str) -> (String, Vec<String>) {
        let mut result = content.to_string();
        let mut removed = Vec::new();
        
        // Application fields to strip/replace
        let patterns = [
            (r"<Application>[^<]*</Application>", "<Application>Document Editor</Application>".to_string(), "Application"),
            (r"<AppVersion>[^<]*</AppVersion>", "<AppVersion>1.0</AppVersion>".to_string(), "AppVersion"),
            (r"<Company>[^<]*</Company>", "<Company></Company>".to_string(), "Company"),
            (r"<Manager>[^<]*</Manager>", "".to_string(), "Manager"),
            (r"<TotalTime>\d+</TotalTime>", "<TotalTime>0</TotalTime>".to_string(), "TotalTime"),
            (r"<Template>[^<]*</Template>", "<Template>Normal</Template>".to_string(), "Template"),
        ];
        
        for (pattern, replacement, field_name) in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&result) {
                    result = re.replace_all(&result, replacement.as_str()).to_string();
                    removed.push(field_name.to_string());
                }
            }
        }
        
        (result, removed)
    }
    
    /// Main entry point - strip metadata from any supported document
    pub fn strip_document(&self, input_path: &str, output_path: &str) -> Result<StrippingResult, String> {
        log::info!("Stripping metadata from: {}", input_path);
        
        match self.detect_type(input_path) {
            DocumentType::Pdf => self.strip_pdf_metadata(input_path, output_path),
            DocumentType::Docx | DocumentType::Xlsx | DocumentType::Pptx |
            DocumentType::Odt | DocumentType::Ods => {
                self.strip_office_metadata(input_path, output_path)
            }
            DocumentType::Unknown => {
                Err(format!("Unsupported document type: {}", input_path))
            }
        }
    }
    
    /// Get fake metadata for reporting
    pub fn get_fake_metadata(&self) -> DocumentMetadata {
        DocumentMetadata {
            title: None,
            author: Some(self.fake_metadata.author.clone()),
            subject: None,
            creator: Some(self.fake_metadata.creator.clone()),
            producer: Some(self.fake_metadata.producer.clone()),
            creation_date: None,
            modification_date: None,
            keywords: None,
            company: Some(self.fake_metadata.company.clone()),
            manager: None,
            revision: Some("1".to_string()),
            total_edit_time: Some("0".to_string()),
        }
    }
    
    /// Create output path in temp directory
    pub fn create_output_path(&self, input_path: &str) -> Result<String, String> {
        let path = Path::new(input_path);
        let filename = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("document");
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("dat");
        
        let temp_dir = dirs::cache_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
            .join("servionx-browser")
            .join("stripped-docs");
        
        std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
        
        let output_path = temp_dir.join(format!("{}_stripped.{}", filename, extension));
        
        Ok(output_path.to_string_lossy().to_string())
    }
}

/// Result of document metadata stripping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrippingResult {
    pub document_type: DocumentType,
    pub original_size: usize,
    pub output_size: usize,
    pub fields_removed: Vec<String>,
    pub fake_metadata_injected: bool,
    pub output_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_document_type_detection() {
        let stripper = DocumentStripper::new();
        
        assert!(matches!(stripper.detect_type("test.pdf"), DocumentType::Pdf));
        assert!(matches!(stripper.detect_type("test.docx"), DocumentType::Docx));
        assert!(matches!(stripper.detect_type("test.doc"), DocumentType::Docx));
        assert!(matches!(stripper.detect_type("test.xlsx"), DocumentType::Xlsx));
        assert!(matches!(stripper.detect_type("test.xls"), DocumentType::Xlsx));
        assert!(matches!(stripper.detect_type("test.pptx"), DocumentType::Pptx));
        assert!(matches!(stripper.detect_type("test.odt"), DocumentType::Odt));
        assert!(matches!(stripper.detect_type("test.txt"), DocumentType::Unknown));
    }
    
    #[test]
    fn test_fake_metadata() {
        let stripper = DocumentStripper::new();
        let fake = stripper.get_fake_metadata();
        
        assert_eq!(fake.author, Some("User".to_string()));
        assert_eq!(fake.creator, Some("Document Editor".to_string()));
        assert_eq!(fake.producer, Some("ServionX Protected".to_string()));
    }
    
    #[test]
    fn test_core_xml_stripping() {
        let stripper = DocumentStripper::new();
        let content = r#"<dc:creator>John Doe</dc:creator><dc:title>Secret Doc</dc:title>"#;
        let (stripped, fields) = stripper.strip_core_xml(content);
        
        assert!(stripped.contains("<dc:creator>User</dc:creator>"));
        assert!(stripped.contains("<dc:title></dc:title>"));
        assert!(!stripped.contains("John Doe"));
        assert!(!stripped.contains("Secret Doc"));
        assert!(fields.contains(&"dc:creator".to_string()));
        assert!(fields.contains(&"dc:title".to_string()));
    }
    
    #[test]
    fn test_app_xml_stripping() {
        let stripper = DocumentStripper::new();
        let content = r#"<Application>Microsoft Word</Application><Company>ACME Corp</Company><TotalTime>120</TotalTime>"#;
        let (stripped, fields) = stripper.strip_app_xml(content);
        
        assert!(stripped.contains("<Application>Document Editor</Application>"));
        assert!(stripped.contains("<Company></Company>"));
        assert!(stripped.contains("<TotalTime>0</TotalTime>"));
        assert!(!stripped.contains("Microsoft Word"));
        assert!(!stripped.contains("ACME Corp"));
        assert!(fields.contains(&"Application".to_string()));
        assert!(fields.contains(&"Company".to_string()));
    }
}
