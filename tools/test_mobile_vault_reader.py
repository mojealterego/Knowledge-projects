import base64
import os
from pathlib import Path
import tempfile
import unittest
from mobile_vault_reader import read_binary_in_chunks, VaultAccessError, check_path

JPG = b'\xff\xd8\xff\xe0' + b'x' * 130
TIFF = b'II*\x00\x08\x00\x00\x00' + b'\x00'*10
class VaultReaderTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root/'pictures').mkdir()
        (self.root/'pictures'/'photo.jpg').write_bytes(JPG)
    def test_jpeg_read(self):
        v=read_binary_in_chunks(self.root, 'pictures/photo.jpg')
        self.assertEqual(v.detected_format, 'jpeg')
        self.assertEqual(base64.b64decode(''.join(x[1] for x in v.chunks)), JPG)
    def test_chunk_offsets_and_padding(self):
        v=read_binary_in_chunks(self.root, 'pictures/photo.jpg',chunk_bytes=33)
        self.assertEqual([x[0] for x in v.chunks],list(range(0,len(JPG),33)))
        self.assertTrue(all(not x[1].endswith('=') for x in v.chunks[:-1]))
    def test_tiff_family_not_assumed_nef(self):
        (self.root/'sample.nef').write_bytes(TIFF)
        self.assertEqual(read_binary_in_chunks(self.root,'sample.nef').detected_format,'tiff_family_unclassified')
    def test_extension_does_not_define_format(self):
        (self.root/'fake.nef').write_bytes(b'not camera data')
        with self.assertRaises(VaultAccessError):read_binary_in_chunks(self.root,'fake.nef')
    def test_png_magic(self):
        (self.root/'a.png').write_bytes(b'\x89PNG\r\n\x1a\n'+b'1'*19)
        self.assertEqual(read_binary_in_chunks(self.root,'a.png').detected_format,'png')
    def test_directory_escape(self):
        with self.assertRaises(VaultAccessError):check_path('../private.jpg')
    def test_absolute_path(self):
        with self.assertRaises(VaultAccessError):check_path('/etc/passwd')
    def test_dotted_component(self):
        with self.assertRaises(VaultAccessError):check_path('pictures/./photo.jpg')
    def test_backslashes(self):
        with self.assertRaises(VaultAccessError):check_path('pictures\\photo.jpg')
    def test_inner_symlink_rejected(self):
        (self.root/'linked').symlink_to(self.root/'pictures',target_is_directory=True)
        with self.assertRaises(VaultAccessError):read_binary_in_chunks(self.root,'linked/photo.jpg')
    def test_file_symlink_rejected(self):
        (self.root/'alias.jpg').symlink_to(self.root/'pictures'/'photo.jpg')
        with self.assertRaises(VaultAccessError):read_binary_in_chunks(self.root,'alias.jpg')
    def test_fifo_rejected_without_block(self):
        os.mkfifo(self.root/'pipe.jpg')
        with self.assertRaises(VaultAccessError):read_binary_in_chunks(self.root,'pipe.jpg')
    def test_filesize_limit(self):
        with self.assertRaises(VaultAccessError):read_binary_in_chunks(self.root,'pictures/photo.jpg',max_bytes=16)
    def test_chunk_alignment(self):
        with self.assertRaises(ValueError):read_binary_in_chunks(self.root,'pictures/photo.jpg',chunk_bytes=1024)
    def test_read_nonexistent(self):
        with self.assertRaises(VaultAccessError):read_binary_in_chunks(self.root,'a.jpg')
    def test_cr2_marker(self):
        (self.root/'sample.raw').write_bytes(b'II*\x00\x10\x00\x00\x00CR\x02\x00' + b'a'*20)
        self.assertEqual(read_binary_in_chunks(self.root,'sample.raw').detected_format,'cr2_tiff')
if __name__ == '__main__':unittest.main()
