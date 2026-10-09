import unittest
from datetime import datetime,timezone
from seeker_evidence_scope_gate import assess

NOW=datetime(2026,10,9,12,tzinfo=timezone.utc)
DIGEST="a"*64
BASE={"case_type":"own_device_incident","source_kind":"owned_device_inventory",
      "authorization_scope_id":"case-owner-approved","owner_consent":True,
      "source_sha256":DIGEST,"network_access":False,
      "contains_personal_identifiers":False,"action":"review_source_provenance",
      "claims_live_device_location":False,"physical_geolocation_from_mac_or_imei":False}
def review(**change):
    x=dict(BASE);x.update(change)
    return assess(x,now=NOW,verified_scope_ids=frozenset({"case-owner-approved"}))
class SeekerEvidenceTests(unittest.TestCase):
    def test_authorized_owned_device_metadata(self):self.assertTrue(review().admissible)
    def test_public_threat_advisory_with_consent(self):
        self.assertTrue(review(case_type="public_entity_threat_intel",source_kind="public_advisory").admissible)
    def test_unconsented_person_scope(self):self.assertIn("unapproved_case_type",review(case_type="third_party_person").reasons)
    def test_explicit_verified_scope(self):self.assertIn("authorization_not_verified",review(authorization_scope_id="guess").reasons)
    def test_subject_consent_required(self):self.assertIn("consent_not_verified",review(owner_consent=False).reasons)
    def test_hash_required(self):self.assertIn("invalid_source_hash",review(source_sha256="source.png").reasons)
    def test_network_denied(self):self.assertIn("network_access_forbidden",review(network_access=True).reasons)
    def test_private_identity_denied(self):self.assertIn("private_identifiers_must_be_redacted",review(contains_personal_identifiers=True).reasons)
    def test_location_tracking_denied(self):self.assertIn("surveillance_or_intrusive_action_forbidden",review(action="location_tracking").reasons)
    def test_contact_enumeration_denied(self):self.assertFalse(review(action="contact_enumeration").admissible)
    def test_hlr_denied(self):self.assertFalse(review(action="ss7_hlr_query").admissible)
    def test_mac_not_physical_location(self):self.assertIn("mac_imei_not_location_proof",review(physical_geolocation_from_mac_or_imei=True).reasons)
    def test_claim_live_location_denied(self):self.assertIn("location_cannot_be_inferred_from_metadata",review(claims_live_device_location=True).reasons)
    def test_no_physical_location_verified(self):self.assertFalse(review().physical_location_verified)
    def test_untrusted_new_source_type_denied(self):self.assertIn("source_requires_new_review",review(source_kind="unknown_tracking_provider").reasons)
    def test_no_external_calls(self):
        import seeker_evidence_scope_gate as m
        self.assertNotIn("requests",m.__dict__)
        self.assertNotIn("subprocess",m.__dict__)
if __name__=="__main__":unittest.main()
