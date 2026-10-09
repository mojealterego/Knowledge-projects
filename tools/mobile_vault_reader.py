"""Local read-only Android/Termux vault primitive; not an MCP server or a phone connector.

No network, subprocess, file modification, or Google Drive access. Linux/POSIX
O_NOFOLLOW + directory-fd traversal prevents symlink and string-prefix escape
for trusted, owner-selected roots. Only reads small bounded files; no raw file
contents are logged. TIFF-based RAW subtypes require additional IFD parsing;
file extension is not used as an authenticated type signature.
"""
from __future__ import annotations
from dataclasses import dataclass
import base64
import os
from pathlib import Path
import stat

MAX_BYTES = 16 * 1024 * 1024
DEFAULT_CHUNK_BYTES = 60 * 1024  # multiple of 3; no base64 padding except end

class VaultAccessError(ValueError):
    pass

@dataclass(frozen=True)
class VaultReport:
    length: int
    detected_format: str
    chunks: tuple[tuple[int, str], ...]

def check_path(relative_path: str) -> tuple[str, ...]:
    if not isinstance(relative_path, str) or not relative_path or '\x00' in relative_path:
        raise VaultAccessError('invalid_relative_path')
    if relative_path.startswith('/') or '\\' in relative_path:
        raise VaultAccessError('absolute_or_platform_path')
    pieces = relative_path.split('/')
    if any(x in ('', '.', '..') for x in pieces):
        raise VaultAccessError('unsafe_path_component')
    return tuple(pieces)

def _detect_magic(prefix: bytes) -> str:
    if prefix.startswith(b'\xff\xd8\xff'):
        return 'jpeg'
    if prefix.startswith(b'\x89PNG\r\n\x1a\n'):
        return 'png'
    if prefix.startswith((b'II*\x00', b'MM\x00*')):
        if len(prefix) >= 12 and prefix[8:10] == b'CR':
            return 'cr2_tiff'
        return 'tiff_family_unclassified'
    raise VaultAccessError('unsupported_or_unverified_binary_signature')

def _safe_open(root: Path, components: tuple[str, ...]) -> int:
    flags_dir = os.O_RDONLY | getattr(os, 'O_DIRECTORY', 0) | getattr(os, 'O_NOFOLLOW', 0)
    flags_file = os.O_RDONLY | getattr(os, 'O_NOFOLLOW', 0) | getattr(os, 'O_NONBLOCK', 0)
    dirs: list[int] = []
    try:
        dirs.append(os.open(root, flags_dir))
        for name in components[:-1]:
            dirs.append(os.open(name, flags_dir, dir_fd=dirs[-1]))
        fd = os.open(components[-1], flags_file, dir_fd=dirs[-1])
        try:
            mode = os.fstat(fd).st_mode
            if not stat.S_ISREG(mode):
                raise VaultAccessError('not_a_regular_file')
            return fd
        except Exception:
            os.close(fd)
            raise
    except OSError as exc:
        raise VaultAccessError('unsafe_or_missing_vault_file') from exc
    finally:
        for d in reversed(dirs):
            os.close(d)

def read_binary_in_chunks(root: str | Path, relative_path: str, *,
                          max_bytes: int = MAX_BYTES,
                          chunk_bytes: int = DEFAULT_CHUNK_BYTES) -> VaultReport:
    if type(max_bytes) is not int or not 1 <= max_bytes <= MAX_BYTES:
        raise ValueError('invalid_max_bytes')
    if type(chunk_bytes) is not int or not 3 <= chunk_bytes <= 1024 * 1024 or chunk_bytes % 3:
        raise ValueError('chunk_size_must_be_multiple_of_three')
    components = check_path(relative_path)
    fd = _safe_open(Path(root), components)
    try:
        info = os.fstat(fd)
        if info.st_size > max_bytes:
            raise VaultAccessError('file_exceeds_limit')
        header = os.read(fd, 16)
        detected = _detect_magic(header)
        os.lseek(fd, 0, os.SEEK_SET)
        chunks = []
        consumed = 0
        while True:
            block = os.read(fd, min(chunk_bytes, max_bytes - consumed + 1))
            if not block:
                break
            consumed += len(block)
            if consumed > max_bytes:
                raise VaultAccessError('file_grew_past_limit')
            chunks.append((consumed - len(block), base64.b64encode(block).decode('ascii')))
        if consumed != info.st_size:
            raise VaultAccessError('file_changed_while_reading')
        return VaultReport(consumed, detected, tuple(chunks))
    finally:
        os.close(fd)
