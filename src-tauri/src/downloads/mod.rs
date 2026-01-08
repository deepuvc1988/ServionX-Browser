//! Downloads Module
//! Smart downloading with video grabbing capability

pub mod smart_downloader;
pub mod video_grabber;
pub mod commands;

pub use smart_downloader::{SmartDownloader, Download, DownloadState};
pub use video_grabber::{VideoGrabber, DetectedMedia, MediaType, QualityLevel};
