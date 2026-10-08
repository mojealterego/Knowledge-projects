import tempfile
import unittest
from pathlib import Path
from zipfile import ZipFile, ZipInfo, ZIP_DEFLATED
from source_bundle_audit import UnsafeBundle, audit_zip


class SourceBundleAuditTests(unittest.TestCase):
    def setUp(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        self.path = Path(tmp.name) / "new_bundle.zip"

    def make(self, members):
        with ZipFile(self.path, "w", compression=ZIP_DEFLATED) as zipout:
            for k, v in members:
                zipout.writestr(k, v)
        return self.path

    def test_hashing_and_no_filename_leak_default(self):
        self.make([("private/file.pdf", b"sample")])
        result = audit_zip(self.path)
        self.assertEqual(result["member_count"], 1)
        self.assertEqual(result["total_uncompressed_bytes"], 6)
        self.assertNotIn("path", result["files"][0])
        self.assertEqual(result["status"], "INVENTORIED_NOT_SEMANTICALLY_REVIEWED")

    def test_filename_opt_in_only(self):
        self.make([("public/a.txt", b"hi")])
        self.assertEqual(audit_zip(self.path, include_names=True)["files"][0]["path"], "public/a.txt")

    def test_same_content_identified(self):
        self.make([("a.pdf", b"same"), ("b.pdf", b"same")])
        self.assertEqual(list(audit_zip(self.path)["duplicate_content_hashes"].values()), [2])

    def test_traversal_rejected(self):
        self.make([("../../not-in-folder.pdf", b"x")])
        with self.assertRaises(UnsafeBundle):
            audit_zip(self.path)

    def test_absolute_path_rejected(self):
        self.make([("/etc/passwd", b"x")])
        with self.assertRaises(UnsafeBundle):
            audit_zip(self.path)

    def test_windows_drive_rejected(self):
        self.make([("C:\\Windows\\path.txt", b"x")])
        with self.assertRaises(UnsafeBundle):
            audit_zip(self.path)

    def test_case_insensitive_collision_rejected(self):
        self.make([("A.pdf", b"x"), ("a.PDF", b"y")])
        with self.assertRaises(UnsafeBundle):
            audit_zip(self.path)

    def test_symlink_rejected(self):
        zinfo = ZipInfo("link")
        zinfo.create_system = 3
        zinfo.external_attr = 0o120777 << 16
        with ZipFile(self.path, "w") as z:
            z.writestr(zinfo, "../../secret")
        with self.assertRaises(UnsafeBundle):
            audit_zip(self.path)

    def test_zip_bomb_ratio_rejected(self):
        self.make([("huge.txt", b"0" * 2000000)])
        with self.assertRaises(UnsafeBundle):
            audit_zip(self.path)

    def test_bad_zip_rejected(self):
        self.path.write_bytes(b"not a zip")
        with self.assertRaises(UnsafeBundle):
            audit_zip(self.path)

    def test_empty_zip_allowed(self):
        self.make([])
        self.assertEqual(audit_zip(self.path)["member_count"], 0)


if __name__ == "__main__":
    unittest.main()
