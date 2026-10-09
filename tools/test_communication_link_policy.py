import unittest
from communication_link_policy import PROTOCOLS, review_link

def check(**change):
    plan={"protocol":"bluetooth_le","purpose":"owned_device_transfer",
          "endpoint_owner_authorized":True,"counterparty_opted_in":True,
          "passive_tracking":False,"network_scan":False,
          "claims_global_device_location":False,"cloud_publication":False}
    plan.update(change)
    return review_link(plan)

class CommunicationPolicyTests(unittest.TestCase):
    def test_owned_device_bluetooth(self): self.assertTrue(check().accepted)
    def test_supported_protocols_are_categorized(self):
        self.assertEqual(len(PROTOCOLS),8)
        self.assertEqual(PROTOCOLS["zenoh"]["transport"],"software_messaging")
    def test_nfc_has_close_proximity_label(self): self.assertEqual(PROTOCOLS["nfc"]["proximity"],"very_near")
    def test_unknown_protocol_denied(self): self.assertIn("unknown_protocol",check(protocol="imaginary").reasons)
    def test_no_device_owner_authority(self): self.assertIn("endpoint_owner_not_authorized",check(endpoint_owner_authorized=False).reasons)
    def test_no_counterparty_consent(self): self.assertIn("counterparty_consent_missing",check(counterparty_opted_in=False).reasons)
    def test_no_passive_tracking(self): self.assertIn("passive_tracking_forbidden",check(passive_tracking=True).reasons)
    def test_no_network_scan(self): self.assertIn("network_scanning_forbidden",check(network_scan=True).reasons)
    def test_no_claim_global_location(self): self.assertIn("protocol_does_not_prove_global_location",check(claims_global_device_location=True).reasons)
    def test_no_auto_cloud_publish(self): self.assertIn("external_data_transfer_requires_separate_approval",check(cloud_publication=True).reasons)
    def test_no_public_tracking_purpose(self): self.assertIn("unapproved_purpose",check(purpose="track_someone_else").reasons)
    def test_uwb_does_not_prove_coordinates(self):
        v=check(protocol="uwb")
        self.assertTrue(v.accepted)
        self.assertFalse(v.has_physical_location_evidence)
    def test_zenoh_not_a_physical_radio(self): self.assertEqual(PROTOCOLS["zenoh"]["proximity"],"network_dependent")
    def test_no_network_call_libraries(self):
        import communication_link_policy as m
        self.assertNotIn("requests",m.__dict__)
        self.assertNotIn("subprocess",m.__dict__)
if __name__=="__main__":unittest.main()
