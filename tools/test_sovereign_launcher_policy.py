"""Static source-contract tests; do NOT substitute for Android SDK build/device test."""
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
APP = ROOT / "projekty" / "125-sovereign-contextual-android-launcher" / "android"
MANIFEST = APP / "app" / "src" / "main" / "AndroidManifest.xml"
MAIN = APP / "app" / "src" / "main" / "java" / "org" / "mojealterego" / "sovereignlauncher" / "MainActivity.kt"
ANDROID = "{http://schemas.android.com/apk/res/android}"


class AndroidLauncherStaticContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.doc = ET.parse(MANIFEST).getroot()
        cls.kt = MAIN.read_text(encoding="utf-8")

    def test_no_sensitive_permissions(self):
        self.assertEqual(self.doc.findall("uses-permission"), [])

    def test_cleartext_network_disabled(self):
        app = self.doc.find("application")
        self.assertEqual(app.get(ANDROID + "usesCleartextTraffic"), "false")

    def test_home_intent_is_exported(self):
        activity = self.doc.find("./application/activity")
        self.assertEqual(activity.get(ANDROID + "exported"), "true")
        categories = {x.get(ANDROID + "name") for x in activity.findall("./intent-filter/category")}
        self.assertIn("android.intent.category.HOME", categories)
        self.assertIn("android.intent.category.DEFAULT", categories)

    def test_only_explicit_launchable_app_visibility(self):
        self.assertEqual(len(self.doc.findall("queries/intent")), 1)
        self.assertEqual(
            self.doc.find("queries/intent/action").get(ANDROID + "name"),
            "android.intent.action.MAIN",
        )
        self.assertEqual(
            self.doc.find("queries/intent/category").get(ANDROID + "name"),
            "android.intent.category.LAUNCHER",
        )

    def test_launch_requires_user_click_and_explicit_component(self):
        self.assertIn("setOnClickListener", self.kt)
        self.assertIn(".setClassName(", self.kt)
        self.assertIn("startActivity(launch)", self.kt)

    def test_no_sensitive_android_service_or_network_api(self):
        for banned in ("UsageStatsManager", "NotificationListenerService", "AccessibilityService",
                       "HttpURLConnection", "requestLocationUpdates", "QUERY_ALL_PACKAGES"):
            with self.subTest(api=banned):
                self.assertNotIn(banned, self.kt)

    def test_build_contains_no_remote_credentials(self):
        for path in (APP / "build.gradle.kts", APP / "app" / "build.gradle.kts"):
            data = path.read_text(encoding="utf-8")
            self.assertNotIn("OPENAI_API_KEY", data)
            self.assertNotIn("sk-proj-", data)


if __name__ == "__main__":
    unittest.main()
