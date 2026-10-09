import unittest
from telephony_topology_gate import review_topology

def safe():
    return dict(network_owner_approved=True,operator_spectrum_authorization_verified=True,
      public_number_allocation_claimed=False,pstn_enabled=True,
      licensed_sip_trunk_contract_verified=True,emergency_call_handling_reviewed=True,
      identity_by_ip_only=False,sip_identity_auth_configured=True,
      exposes_privileged_containers=False,host_network_for_all_services=False,
      secrets_in_plaintext=False,container_images=['mongo:8.0','asterisk:21.0'],
      subscriber_count=2,subscriber_subnet='10.20.0.0/24',subscriber_ips=['10.20.0.11','10.20.0.12'],
      actual_device_or_radio_tested=True)

def review(**change):
    p=safe();p.update(change);return review_topology(p)
class TelephonyTests(unittest.TestCase):
    def test_approved_plan_is_structurally_valid(self):self.assertTrue(review().deployable)
    def test_pdf_ip_only_unsafe(self):self.assertIn('source_ip_is_not_subscriber_identity',review(identity_by_ip_only=True).findings)
    def test_no_operator_authorization(self):self.assertFalse(review(operator_spectrum_authorization_verified=False).deployable)
    def test_no_own_numbers(self):self.assertIn('public_numbers_cannot_be_self_assigned',review(public_number_allocation_claimed=True).findings)
    def test_no_pstn_contract(self):self.assertIn('pstn_interconnect_not_licensed',review(licensed_sip_trunk_contract_verified=False).findings)
    def test_emergency_calling(self):self.assertFalse(review(emergency_call_handling_reviewed=False).deployable)
    def test_no_privileged_containers(self):self.assertIn('privileged_container_forbidden',review(exposes_privileged_containers=True).findings)
    def test_no_host_network_wide(self):self.assertFalse(review(host_network_for_all_services=True).deployable)
    def test_no_plaintext(self):self.assertFalse(review(secrets_in_plaintext=True).deployable)
    def test_no_latest(self):self.assertIn('container_image_unpinned',review(container_images=['tirmar/freepbx:latest']).findings)
    def test_missing_sip_auth(self):self.assertFalse(review(sip_identity_auth_configured=False).deployable)
    def test_no_radio_test(self):self.assertIn('no_verified_device_integration',review(actual_device_or_radio_tested=False).findings)
    def test_no_duplicate_ips(self):self.assertFalse(review(subscriber_ips=['10.20.0.11','10.20.0.11']).deployable)
    def test_bad_cidr(self):self.assertFalse(review(subscriber_subnet='10.20.0.1/24').deployable)
    def test_not_in_subnet(self):self.assertFalse(review(subscriber_ips=['8.8.8.8','10.20.0.12']).deployable)
    def test_wrong_count(self):self.assertFalse(review(subscriber_count=4).deployable)
    def test_not_actual_deploy(self):
        import telephony_topology_gate as m
        self.assertNotIn('subprocess',m.__dict__)
        self.assertNotIn('requests',m.__dict__)
if __name__=='__main__':unittest.main()
