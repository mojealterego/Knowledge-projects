import tempfile
import unittest
from pathlib import Path
from project_id_gate import existing_claims


class ProjectIdGateTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "projekty").mkdir()

    def test_detects_existing_markdown_owner(self):
        (self.root / "projekty" / "122-chemia.md").write_text("# CHEMIA")
        self.assertEqual(existing_claims(self.root, 122), ["projekty/122-chemia.md"])

    def test_detects_existing_directory_owner(self):
        p = self.root / "projekty" / "122-chemia"
        p.mkdir()
        (p / "README.md").write_text("# CHEMIA")
        self.assertEqual(existing_claims(self.root, 122), ["projekty/122-chemia"])

    def test_detects_collision_of_distinct_products(self):
        for slug in ("122-chemia", "122-gas"):
            p = self.root / "projekty" / slug
            p.mkdir()
            (p / "README.md").write_text(f"# {slug}")
        self.assertEqual(len(existing_claims(self.root, 122)), 2)

    def test_available_number(self):
        (self.root / "projekty" / "122-chemia.md").write_text("# CHEMIA")
        self.assertEqual(existing_claims(self.root, 123), [])

    def test_prefix_not_confused_with_higher_number(self):
        (self.root / "projekty" / "1221-chemia.md").write_text("# other")
        self.assertEqual(existing_claims(self.root, 122), [])

    def test_does_not_count_nonproject_dirs(self):
        (self.root / "projekty" / "124-staging").mkdir()
        self.assertEqual(existing_claims(self.root, 124), [])

    def test_invalid_id_rejected(self):
        with self.assertRaises(ValueError):
            existing_claims(self.root, -1)


if __name__ == "__main__":
    unittest.main()
