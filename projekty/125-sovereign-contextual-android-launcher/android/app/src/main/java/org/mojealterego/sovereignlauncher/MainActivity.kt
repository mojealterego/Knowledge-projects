package org.mojealterego.sovereignlauncher

import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Intent
import android.graphics.Color
import android.os.Bundle
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import android.widget.Toast

/**
 * Minimal, local and deterministic HOME app selector.
 * No network, notifications, usage history, location, accessibility or AI.
 */
class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(Color.WHITE)
            setPadding(24, 32, 24, 20)
        }
        val header = TextView(this).apply {
            text = "Sovereign Launcher — wersja demonstracyjna"
            textSize = 21f
            setTextColor(Color.BLACK)
        }
        root.addView(header)
        root.addView(TextView(this).apply {
            text = "Aplikacje są wyświetlane alfabetycznie. Nic nie uruchamia się bez dotknięcia przycisku."
            textSize = 15f
            setTextColor(Color.DKGRAY)
        })
        val scroll = ScrollView(this)
        val buttons = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
        }
        scroll.addView(buttons)
        root.addView(scroll, LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT, 0, 1f
        ))

        val query = Intent(Intent.ACTION_MAIN).addCategory(Intent.CATEGORY_LAUNCHER)
        @Suppress("DEPRECATION")
        val installed = packageManager.queryIntentActivities(query, 0)
            .filter { it.activityInfo.packageName != packageName }
            .distinctBy { "${it.activityInfo.packageName}/${it.activityInfo.name}" }
            .sortedBy { it.loadLabel(packageManager).toString().lowercase() }
        for (app in installed) {
            val name = app.loadLabel(packageManager).toString()
            val btn = Button(this).apply {
                text = name
                isAllCaps = false
                contentDescription = "Otwórz aplikację: $name"
                setOnClickListener {
                    try {
                        val launch = Intent(Intent.ACTION_MAIN)
                            .addCategory(Intent.CATEGORY_LAUNCHER)
                            .setClassName(app.activityInfo.packageName, app.activityInfo.name)
                            .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                        startActivity(launch)
                    } catch (_: ActivityNotFoundException) {
                        Toast.makeText(this@MainActivity, "Aplikacja niedostępna", Toast.LENGTH_SHORT).show()
                    } catch (_: SecurityException) {
                        Toast.makeText(this@MainActivity, "Brak uprawnień do uruchomienia", Toast.LENGTH_SHORT).show()
                    }
                }
            }
            buttons.addView(btn, LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT
            ))
        }
        if (installed.isEmpty()) {
            buttons.addView(TextView(this).apply {
                text = "Brak aplikacji możliwych do wyświetlenia."
            })
        }
        setContentView(root)
    }
}
