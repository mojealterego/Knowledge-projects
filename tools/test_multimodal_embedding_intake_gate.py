import unittest
from multimodal_embedding_intake_gate import qualify_multimodal_intake

DIGEST = "a" * 64


def good():
    return {
        "model": "gemini-embedding-2",
        "provider": "gemini_api",
        "output_dimensions": 768,
        "task": "retrieval_document",
        "source_sha256": DIGEST,
        "classification": "public",
        "remote_processing_opt_in": True,
        "contains_credentials": False,
        "contains_nonconsensual_personal_data": False,
        "items": [{"modality": "text", "units": 240}],
    }


class GeminiEmbeddingIntakeTests(unittest.TestCase):
    def review(self, m, approved=frozenset()):
        return qualify_multimodal_intake(m, externally_approved_source_hashes=approved)

    def bad(self, changes, reason, approved=frozenset()):
        m = good()
        m.update(changes)
        decision = self.review(m, approved)
        self.assertFalse(decision.qualified)
        self.assertIn(reason, decision.reasons)

    def test_public_small_text(self):
        self.assertTrue(self.review(good()).qualified)

    def test_model_name_pinned(self):
        self.bad({"model": "gemini-3.8-flash"}, "unsupported_model")

    def test_unknown_provider(self):
        self.bad({"provider": "example.invalid"}, "unknown_provider")

    def test_dimension_is_number_not_boolean(self):
        self.bad({"output_dimensions": True}, "invalid_dimensions")

    def test_large_dimension(self):
        self.bad({"output_dimensions": 4096}, "invalid_dimensions")

    def test_minimum_dimension(self):
        m = good();m["output_dimensions"] = 128
        self.assertTrue(self.review(m).qualified)

    def test_invalid_hash(self):
        self.bad({"source_sha256": "not-a-hash"}, "invalid_source_sha256")

    def test_no_remote_opt_in(self):
        self.bad({"remote_processing_opt_in": False}, "remote_processing_not_authorized")

    def test_private_data_needs_independent_approval(self):
        self.bad({"classification": "restricted"}, "sensitive_source_not_independently_approved")

    def test_private_data_approved_by_external_system(self):
        m = good();m["classification"] = "restricted"
        self.assertTrue(self.review(m, frozenset({DIGEST})).qualified)

    def test_pdf_limit(self):
        m = good();m["items"] = [{"modality": "pdf", "units": 7}]
        self.assertIn("pdf_limit_exceeded", self.review(m).reasons)

    def test_video_limit(self):
        m = good();m["items"] = [{"modality": "video", "units": 121}]
        self.assertIn("video_limit_exceeded", self.review(m).reasons)

    def test_audio_limit(self):
        m = good();m["items"] = [{"modality": "audio", "units": 181}]
        self.assertIn("audio_limit_exceeded", self.review(m).reasons)

    def test_six_images_cap(self):
        m = good();m["items"] = [{"modality": "image", "units": 4},{"modality": "image", "units": 3}]
        self.assertIn("image_limit_exceeded", self.review(m).reasons)

    def test_text_limit(self):
        m = good();m["items"] = [{"modality": "text", "units": 8193}]
        self.assertIn("text_limit_exceeded", self.review(m).reasons)

    def test_missing_measured_count(self):
        m = good();m["items"] = [{"modality": "video", "units": None}]
        self.assertIn("unmeasured_or_invalid_units", self.review(m).reasons)

    def test_reject_credentials(self):
        self.bad({"contains_credentials": True}, "credentials_must_not_leave_client")

    def test_reject_unconsented_personal_data(self):
        self.bad({"contains_nonconsensual_personal_data": True}, "personal_data_without_consent")

    def test_empty_items(self):
        self.bad({"items": []}, "missing_items")

    def test_no_network_runtime_imports(self):
        import multimodal_embedding_intake_gate as m
        self.assertNotIn("requests", m.__dict__)
        self.assertNotIn("google", m.__dict__)


if __name__ == "__main__":
    unittest.main()
