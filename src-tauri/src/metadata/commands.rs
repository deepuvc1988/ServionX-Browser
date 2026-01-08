// Metadata Commands
// Tauri commands for metadata operations

use tauri::State;
use crate::metadata::{MetadataStripper, FileMetadata, StrippedFile};

/// Get metadata from a file
#[tauri::command]
pub fn get_file_metadata(
    stripper: State<MetadataStripper>,
    file_path: String,
) -> Result<FileMetadata, String> {
    stripper.get_metadata(&file_path)
}

/// Strip metadata from a file
#[tauri::command]
pub fn strip_file_metadata(
    stripper: State<MetadataStripper>,
    file_path: String,
    inject_fake: bool,
) -> Result<StrippedFile, String> {
    stripper.strip_metadata(&file_path, inject_fake)
}
