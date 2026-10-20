use std::io::{Cursor, Write};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};
use zip_warden_agent::agent::{audit_bytes, AuditError};

fn bundle(rows: &[(&str, &[u8])], compression: CompressionMethod) -> Vec<u8> {
    let mut output = Cursor::new(Vec::new());
    {
        let mut zip = ZipWriter::new(&mut output);
        for (path, bytes) in rows {
            zip.start_file(*path, FileOptions::default().compression_method(compression)).unwrap();
            zip.write_all(bytes).unwrap();
        }
        zip.finish().unwrap();
    }
    output.into_inner()
}
#[test]
fn hashes_real_uncompressed_bytes_without_leaking_names() {
    let b = bundle(&[("private/document.pdf", b"sample")], CompressionMethod::Stored);
    let out = audit_bytes(&b, false).unwrap();
    assert_eq!(out.status, "INVENTORIED_NOT_SEMANTICALLY_REVIEWED");
    assert_eq!(out.member_count, 1);
    assert_eq!(out.total_uncompressed_bytes, 6);
    assert_eq!(out.files[0].sha256, "af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d026e0eaeb5eaa2f5e7aebcdeba");
    assert_eq!(out.files[0].path, None);
    assert_eq!(out.files[0].extension, ".pdf");
}
#[test]
fn filename_is_only_shown_when_explicitly_opted_in() {
    let b=bundle(&[("public/a.txt",b"hi")],CompressionMethod::Stored);
    assert_eq!(audit_bytes(&b,true).unwrap().files[0].path.as_deref(),Some("public/a.txt"));
    assert_eq!(audit_bytes(&b,false).unwrap().files[0].path,None);
}
#[test]
fn traversal_and_case_collisions_are_forbidden() {
    let b=bundle(&[("../../secret",b"x")],CompressionMethod::Stored);
    assert_eq!(audit_bytes(&b,false).err(),Some(AuditError::ArchivePathTraversal));
    let b=bundle(&[("A.pdf",b"a"),("a.PDF",b"b")],CompressionMethod::Stored);
    assert_eq!(audit_bytes(&b,false).err(),Some(AuditError::DuplicatePathCaseInsensitive));
}
#[test]
fn duplicate_content_is_counted_and_no_files_are_extracted() {
    let b=bundle(&[("a.pdf",b"same"),("b.pdf",b"same")],CompressionMethod::Stored);
    let out=audit_bytes(&b,false).unwrap();
    assert_eq!(out.duplicate_content_hashes.len(),1);
    assert_eq!(*out.duplicate_content_hashes.values().next().unwrap(),2);
}
#[test]
fn malformed_zip_and_high_compression_ratio_fail_closed() {
    assert_eq!(audit_bytes(b"not a ZIP",false).err(),Some(AuditError::CorruptZip));
    let b=bundle(&[("large.txt",&vec![0u8;2_000_000])],CompressionMethod::Deflated);
    assert_eq!(audit_bytes(&b,false).err(),Some(AuditError::SuspiciousCompressionRatio));
}
