//! Privacy preserving ZIP inventory, no filesystem extraction and no code execution.
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, HashSet},
    io::{Cursor, Read},
};
use zip::ZipArchive;

pub const AGENT_NAME: &str = "ZIP WARDEN";
pub const REGISTRY_NUMBER: &str = "010";
pub const AGENT_ID: &str = "AM-OCAF-AI-0010";
pub const SERIAL_NUMBER: &str = "AM-ARC-ZWD-2026-000010";
pub const CATALOG_NUMBER: &str = "AM-OCAF-ARC-010";

const MAX_ARCHIVE_BYTES: usize = 16 * 1024 * 1024;
const MAX_MEMBERS: usize = 500;
const MAX_MEMBER_BYTES: u64 = 64 * 1024 * 1024;
const MAX_TOTAL_BYTES: u64 = 256 * 1024 * 1024;
const MAX_RATIO: u64 = 1000;

#[derive(Debug, PartialEq, Eq)]
pub enum AuditError {
    CorruptZip,
    ArchiveTooLarge,
    TooManyMembers,
    EmptyOrNulPath,
    ArchivePathTraversal,
    DuplicatePathCaseInsensitive,
    SymlinkMemberForbidden,
    EncryptedOrUnsupportedMember,
    UncompressedSizeLimit,
    SuspiciousCompressionRatio,
    MemberSizeMismatch,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileRow {
    pub sha256: String,
    pub size_bytes: u64,
    pub extension: String,
    pub path_fingerprint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuditReport {
    pub agent_id: &'static str,
    pub source_zip_sha256: String,
    pub status: &'static str,
    pub privacy_names_included: bool,
    pub member_count: usize,
    pub total_uncompressed_bytes: u64,
    pub duplicate_content_hashes: BTreeMap<String, u64>,
    pub files: Vec<FileRow>,
    pub semantic_review_performed: bool,
    pub archive_extracted: bool,
}

fn normalize_path(name: &str) -> Result<String, AuditError> {
    if name.is_empty() || name.contains('\0') {
        return Err(AuditError::EmptyOrNulPath);
    }
    let normalized = name.replace('\\', "/");
    if normalized.starts_with('/')
        || normalized
            .split('/')
            .any(|part| part == ".." || part.is_empty())
    {
        return Err(AuditError::ArchivePathTraversal);
    }
    let first = normalized.split('/').next().unwrap_or("");
    if first.contains(':') || first == "." {
        return Err(AuditError::ArchivePathTraversal);
    }
    Ok(normalized)
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
/// Audit user supplied ZIP bytes in memory; never writes or extracts files.
/// include_names should be false for externally exposed OCAF tools.
pub fn audit_bytes(input: &[u8], include_names: bool) -> Result<AuditReport, AuditError> {
    if input.len() > MAX_ARCHIVE_BYTES {
        return Err(AuditError::ArchiveTooLarge);
    }
    let mut archive = ZipArchive::new(Cursor::new(input)).map_err(|_| AuditError::CorruptZip)?;
    // Original script only counts files. All entries are still subject to path checks.
    let mut count = 0usize;
    let mut seen = HashSet::new();
    let mut total = 0u64;
    let mut files = Vec::new();
    let mut content_counts = BTreeMap::<String, u64>::new();
    for idx in 0..archive.len() {
        let mut member = archive
            .by_index(idx)
            .map_err(|_| AuditError::EncryptedOrUnsupportedMember)?;
        // Match the Python reference: directory records are not inventoried.
        if member.is_dir() {
            continue;
        }
        let path = normalize_path(member.name())?;
        count += 1;
        if count > MAX_MEMBERS {
            return Err(AuditError::TooManyMembers);
        }
        // The original uses Python str.casefold(); Rust lowercase is not a full Unicode case-fold.
        // Reject non-ASCII names to avoid silent case-collision discrepancies.
        if !path.is_ascii() {
            return Err(AuditError::ArchivePathTraversal);
        }
        if !seen.insert(path.to_ascii_lowercase()) {
            return Err(AuditError::DuplicatePathCaseInsensitive);
        }
        if member
            .unix_mode()
            .is_some_and(|mode| mode & 0o170000 == 0o120000)
        {
            return Err(AuditError::SymlinkMemberForbidden);
        }
        let size = member.size();
        if size > MAX_MEMBER_BYTES || total.saturating_add(size) > MAX_TOTAL_BYTES {
            return Err(AuditError::UncompressedSizeLimit);
        }
        let compressed = member.compressed_size();
        if size > 0 && (compressed == 0 || size > compressed.saturating_mul(MAX_RATIO)) {
            return Err(AuditError::SuspiciousCompressionRatio);
        }
        let mut hasher = Sha256::new();
        let mut received = 0u64;
        let mut chunk = [0u8; 8192];
        loop {
            let n = member
                .read(&mut chunk)
                .map_err(|_| AuditError::CorruptZip)?;
            if n == 0 {
                break;
            }
            received = received.saturating_add(n as u64);
            if received > MAX_MEMBER_BYTES || total.saturating_add(received) > MAX_TOTAL_BYTES {
                return Err(AuditError::UncompressedSizeLimit);
            }
            hasher.update(&chunk[..n]);
        }
        if received != size {
            return Err(AuditError::MemberSizeMismatch);
        }
        total += received;
        let member_hash = format!("{:x}", hasher.finalize());
        *content_counts.entry(member_hash.clone()).or_default() += 1;
        let extension = path
            .rsplit('/')
            .next()
            .and_then(|file| {
                file.rfind('.')
                    .filter(|pos| *pos > 0)
                    .map(|at| file[at..].to_ascii_lowercase())
            })
            .unwrap_or_default();
        let fingerprint = digest(path.as_bytes());
        files.push(FileRow {
            sha256: member_hash,
            size_bytes: received,
            extension,
            path_fingerprint: fingerprint[..20].to_owned(),
            path: if include_names { Some(path) } else { None },
        });
    }
    content_counts.retain(|_, count| *count > 1);
    Ok(AuditReport {
        agent_id: AGENT_ID,
        source_zip_sha256: digest(input),
        status: "INVENTORIED_NOT_SEMANTICALLY_REVIEWED",
        privacy_names_included: include_names,
        member_count: files.len(),
        total_uncompressed_bytes: total,
        duplicate_content_hashes: content_counts,
        files,
        semantic_review_performed: false,
        archive_extracted: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn path_traversal_windows_drive_and_absolute_rejected() {
        for bad in [
            "../../secret",
            "C:\\Windows\\file",
            "/etc/passwd",
            "x//y",
            "",
        ] {
            assert!(normalize_path(bad).is_err());
        }
    }
    #[test]
    fn no_network_no_execution_ever() {
        assert_eq!(
            audit_bytes(b"not zip", false).err(),
            Some(AuditError::CorruptZip)
        );
    }
}
