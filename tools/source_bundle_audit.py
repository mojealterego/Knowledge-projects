"""Offline, read-only, privacy-preserving inventory of user-supplied ZIP bundles.

No network requests. Does not upload, extract, publish or execute content.
File names are omitted by default to avoid inadvertently publishing personal data.
The output represents file inventory, not semantic knowledge extraction.
"""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path, PurePosixPath
import stat
import zipfile

MAX_MEMBERS = 500
MAX_MEMBER_BYTES = 750 * 1024 * 1024
MAX_TOTAL_BYTES = 3 * 1024 * 1024 * 1024
MAX_RATIO = 1000
CHUNK = 1024 * 1024


class UnsafeBundle(ValueError):
    """Invalid archive or archive that exceeds intake safety limits."""


def _safe_name(name: str) -> str:
    if not name or "\x00" in name:
        raise UnsafeBundle("empty_or_nul_path")
    p = name.replace("\\", "/")
    parts = PurePosixPath(p).parts
    if p.startswith("/") or any(part in ("..", "") for part in p.split("/")) or ":" in parts[0]:
        raise UnsafeBundle("archive_path_traversal")
    if not parts or parts[0] == ".":
        raise UnsafeBundle("invalid_member_path")
    return p


def audit_zip(archive: str | Path, *, include_names: bool = False) -> dict:
    """Hash contained bytes without extraction or revealing paths.

    Denies traversal, symlinks, encryption, zip bombs, duplicate paths,
    member/total size excess. Hashes actual decompressed member content.
    """
    archive = Path(archive)
    if archive.is_symlink() or not archive.is_file():
        raise UnsafeBundle("source_is_not_regular_file")
    archive_sha = hashlib.sha256()
    with archive.open("rb") as stream:
        for chunk in iter(lambda: stream.read(CHUNK), b""):
            archive_sha.update(chunk)
    rows: list[dict] = []
    seen_names: set[str] = set()
    total = 0
    try:
        with zipfile.ZipFile(archive, "r", allowZip64=True) as z:
            infos = [f for f in z.infolist() if not f.is_dir()]
            if len(infos) > MAX_MEMBERS:
                raise UnsafeBundle("too_many_members")
            for f in infos:
                normalized = _safe_name(f.filename)
                folded = normalized.casefold()
                if folded in seen_names:
                    raise UnsafeBundle("duplicate_path_case_insensitive")
                seen_names.add(folded)
                mode = (f.external_attr >> 16) & 0xFFFF
                if stat.S_IFMT(mode) == stat.S_IFLNK:
                    raise UnsafeBundle("symlink_member_forbidden")
                if f.flag_bits & 0x01:
                    raise UnsafeBundle("encrypted_member_forbidden")
                if f.file_size > MAX_MEMBER_BYTES or total + f.file_size > MAX_TOTAL_BYTES:
                    raise UnsafeBundle("uncompressed_size_limit")
                if f.file_size > 0 and (f.compress_size == 0 or f.file_size / f.compress_size > MAX_RATIO):
                    raise UnsafeBundle("suspicious_compression_ratio")
                digest = hashlib.sha256()
                received = 0
                with z.open(f, "r") as stream:
                    while chunk := stream.read(CHUNK):
                        received += len(chunk)
                        if received > MAX_MEMBER_BYTES or total + received > MAX_TOTAL_BYTES:
                            raise UnsafeBundle("uncompressed_size_limit")
                        digest.update(chunk)
                if received != f.file_size:
                    raise UnsafeBundle("member_size_mismatch")
                total += received
                row = {
                    "sha256": digest.hexdigest(),
                    "size_bytes": received,
                    "extension": PurePosixPath(normalized).suffix.lower(),
                    "path_fingerprint": hashlib.sha256(normalized.encode("utf-8")).hexdigest()[:20],
                }
                if include_names:
                    row["path"] = normalized
                rows.append(row)
    except (zipfile.BadZipFile, EOFError, OSError) as err:
        raise UnsafeBundle("corrupt_or_unreadable_zip") from err
    duplicate_sha = {h: count for h, count in Counter(row["sha256"] for row in rows).items() if count > 1}
    return {
        "source_zip_sha256": archive_sha.hexdigest(),
        "status": "INVENTORIED_NOT_SEMANTICALLY_REVIEWED",
        "privacy_names_included": include_names,
        "member_count": len(rows),
        "total_uncompressed_bytes": total,
        "duplicate_content_hashes": duplicate_sha,
        "files": rows,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Offline ZIP inventory (no extraction/upload)")
    parser.add_argument("archive", type=Path)
    parser.add_argument("--include-names", action="store_true", help="ONLY for local/private reports; may expose personal data")
    args = parser.parse_args()
    try:
        report = audit_zip(args.archive, include_names=args.include_names)
    except UnsafeBundle as err:
        print(json.dumps({"status": "REJECTED", "error": str(err)}))
        return 1
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
