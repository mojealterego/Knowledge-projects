import unittest
from untrusted_agent_source_review import inspect_text

class AgentTextTriageTests(unittest.TestCase):
    def test_shell_async_signal(self):
        self.assertIn('unbounded_shell',inspect_text('await asyncio.create_subprocess_shell(command)').signal_names)
    def test_network_scan(self):self.assertIn('active_network_recon',inspect_text('nmap -sV target').signal_names)
    def test_telecom_recon(self):self.assertIn('active_network_recon',inspect_text('run SS7 query').signal_names)
    def test_overt_bypass(self):self.assertIn('safety_bypass',inspect_text('bypass firewall').signal_names)
    def test_polish_covert(self):self.assertIn('covert_surveillance',inspect_text('inwigilowanie osoby').signal_names)
    def test_invalid_python(self):
        r=inspect_text('--- KONFIGURACJA ---',python_source=True)
        self.assertFalse(r.syntax_parseable_as_python)
        self.assertIn('invalid_python_syntax',r.signal_names)
    def test_syntactically_valid_not_automatically_safe(self):
        r=inspect_text('print(1)',python_source=True)
        self.assertTrue(r.syntax_parseable_as_python)
        self.assertFalse(r.safe_to_execute_automatically)
    def test_never_returns_execution_permission(self):self.assertFalse(inspect_text('').safe_to_execute_automatically)
    def test_bounded_length(self):
        with self.assertRaises(ValueError):inspect_text('a'*1_000_001)
    def test_wrong_type(self):
        with self.assertRaises(TypeError):inspect_text(None)
    def test_zero_day_claim(self):self.assertIn('intrusion_intent',inspect_text('zero-day exploit').signal_names)
    def test_unbounded_loop(self):self.assertIn('unbounded_autonomy',inspect_text('while True:').signal_names)
    def test_not_executor(self):
        import untrusted_agent_source_review as m
        self.assertNotIn('subprocess',m.__dict__)
        self.assertNotIn('requests',m.__dict__)
if __name__=='__main__':unittest.main()
