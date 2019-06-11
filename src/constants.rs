// CLI Parameter error messages
pub const QS3_DEFAULT_HELP_MESSAGE: &str = "Use `qs3 -h` for help";
pub const FILE_NOT_FOUND_ERROR: &str = "Invalid file path";

// S3 error messages
pub const S3_UPLOAD_ID_INVALID: &str = "Invalid upload ID";

// Multipart upload error messages
pub const S3_MULTI_PART_UPLOAD_ERROR: &str = "Error issuing a multipart upload request";
pub const LARGE_FILE_BUFFER_FILL_ERROR: &str = "Large file buffer fill error";
pub const S3_ABORT_MULTI_PART_UPLOAD_ERROR: &str = "Error aborting multipart file upload request";
pub const S3_PART_UPLOAD_REQUEST_ERROR: &str = "Error uploading part #: ";
pub const S3_COMPLETED_MULTI_PART_UPLOAD_REQUEST_ERROR: &str = "Error completing the multipart upload rquest";

// File size values
pub const LARGE_FILE_BYTES_THRESHOLD: u64 = 104857600; // 104MB
pub const AWS_MIN_PART_SIZE: u64 = 5242880; // 5MB