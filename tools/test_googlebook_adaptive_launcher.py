"""Static contract for Googlebook-responsive native Views launcher (NOT device tests).

The upstream Sept 22 2026 article describes Compose/Navigation 3 APIs. P125
deliberately keeps its existing zero-extra-dependency Views scaffold; tests
assert only its actually implemented responsive portion and privacy boundary.
"""
import re
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
APP = ROOT / "projekty" / "125-sovereign-contextual-android-launcher" / "android"
SOURCE = APP / "app" / "src" / "main"
KOTLIN = SOURCE / "java" / "org" / "mojealterego" / "sovereignlauncher" / "MainActivity.kt"
XML = SOURCE / "AndroidManifest.xml"
STRINGS = SOURCE / "res" / "values" / "strings.xml"
ANDROID = "{http://schemas.android.com/apk/res/android}"


class GooglebookAdaptiveSourceContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.code = KOTLIN.read_text(encoding="utf-8")
        cls.manifest = ET.parse(XML).getroot()
        cls.strings = {
            el.attrib["name"]: el.text
            for el in ET.parse(STRINGS).getroot().findall("string")
        }

    def test_current_window_width_triggers_reflow(self):
        self.assertIn("override fun onSizeChanged", self.code)
        self.assertIn("applyWindowWidthDp(windowWidthDp)", self.code)

    def test_source_uses_window_not_physical_screen_width(self):
        self.assertIn("(w / resources.displayMetrics.density).toInt()", self.code)
        self.assertNotIn("defaultDisplay", self.code)
        self.assertNotIn("getRealSize", self.code)
        self.assertNotIn("widthPixels", self.code)

    def test_expanded_840dp_boundary_is_explicit(self):
        self.assertIn("windowWidthDp >= 840", self.code)

    def test_reflow_switches_between_two_panes_and_single_list(self):
        self.assertIn("LinearLayout.HORIZONTAL", self.code)
        self.assertIn("LinearLayout.VERTICAL", self.code)
        self.assertIn("detailPanel.visibility = View.VISIBLE", self.code)
        self.assertIn("detailPanel.visibility = View.GONE", self.code)

    def test_panels_are_reused_without_recreating_buttons(self):
        self.assertEqual(self.code.count("contentRow.addView(appScroll"), 1)
        self.assertEqual(self.code.count("contentRow.addView(detailPanel"), 1)
        self.assertIn("contentRow.requestLayout()", self.code)

    def test_uses_keyboard_focusable_controls(self):
        self.assertIn("isFocusable = true // D-pad", self.code)
        self.assertIn("minHeight = dp(48)", self.code)
        self.assertIn("contentDescription = getString", self.code)

    def test_explicit_user_action_still_required(self):
        self.assertIn("setOnClickListener", self.code)
        self.assertIn("startActivity(launch)", self.code)
        self.assertIn(".setClassName(", self.code)
        self.assertEqual(self.code.count("startActivity(launch)"), 1)

    def test_selection_survives_activity_state_recreation(self):
        self.assertIn("savedInstanceState?.getString(KEY_LAST_SELECTED)", self.code)
        self.assertIn("outState.putString(KEY_LAST_SELECTED, selectedApp)", self.code)

    def test_manifest_requests_resizable_home_activity(self):
        activity = self.manifest.find("./application/activity")
        self.assertEqual(activity.get(ANDROID + "resizeableActivity"), "true")
        self.assertEqual(activity.get(ANDROID + "exported"), "true")
        categories = {x.get(ANDROID + "name") for x in activity.findall("./intent-filter/category")}
        self.assertIn("android.intent.category.HOME", categories)

    def test_no_sensitive_permissions_or_network(self):
        self.assertEqual(self.manifest.findall("uses-permission"), [])
        self.assertEqual(self.manifest.find("application").get(
            ANDROID + "usesCleartextTraffic"), "false")
        for banned in ("UsageStatsManager", "AccessibilityService", "NotificationListenerService",
                       "HttpURLConnection", "requestLocationUpdates", "QUERY_ALL_PACKAGES",
                       "Socket(", "INTERNET"):
            with self.subTest(banned=banned):
                self.assertNotIn(banned, self.code)

    def test_resources_cover_every_referenced_string(self):
        names = set(re.findall(r"R\.string\.([A-Za-z0-9_]+)", self.code))
        self.assertTrue(names)
        self.assertFalse(names - set(self.strings), f"missing: {names - set(self.strings)}")

    def test_no_stub_claiming_compose_or_googlebook_handoff(self):
        self.assertNotIn("HandoffActivityData", self.code)
        self.assertNotIn("ListDetailSceneStrategy", self.code)
        self.assertNotIn("SupportingPaneSceneStrategy", self.code)
        self.assertNotIn("NavDisplay", self.code)

    def test_responsiveness_does_not_grant_agent_control(self):
        self.assertNotIn("AccessibilityManager", self.code)
        self.assertNotIn("Settings.Secure", self.code)
        self.assertNotIn("UsageStatsManager", self.code)


if __name__ == "__main__":
    unittest.main()
