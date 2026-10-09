package org.mojealterego.sovereignlauncher

import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Intent
import android.graphics.Color
import android.os.Bundle
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import android.widget.Toast

/**
 * Android HOME entry point. Uses classic Android Views with no new libraries.
 *
 * The available WINDOW width (not display width) decides whether a dedicated
 * supporting pane is visible. onSizeChanged() responds to freeform resizing.
 * This is a minimal responsive scaffold, not a Compose Navigation 3 app.
 *
 * No network, usage history, notifications, Accessibility Service or AI.
 * Apps are opened only by explicit user button activation.
 */
class MainActivity : Activity() {
    private lateinit var contentRow: LinearLayout
    private lateinit var appScroll: ScrollView
    private lateinit var detailPanel: LinearLayout
    private lateinit var selectedStatus: TextView
    private lateinit var detailLabel: TextView
    private var selectedApp: String? = null
    private var lastExpanded: Boolean? = null

    private fun dp(value: Int): Int =
        (value * resources.displayMetrics.density + 0.5f).toInt()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        selectedApp = savedInstanceState?.getString(KEY_LAST_SELECTED)

        val root = object : LinearLayout(this) {
            override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
                super.onSizeChanged(w, h, oldw, oldh)
                if (w > 0) {
                    // w is THIS app window's available width in pixels.
                    val windowWidthDp = (w / resources.displayMetrics.density).toInt()
                    applyWindowWidthDp(windowWidthDp)
                }
            }
        }.apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(Color.WHITE)
            setPadding(dp(16), dp(16), dp(16), dp(12))
        }

        val title = TextView(this).apply {
            text = getString(R.string.launcher_title)
            textSize = 22f
            setTextColor(Color.BLACK)
            setPadding(0, 0, 0, dp(8))
        }
        root.addView(title)

        val description = TextView(this).apply {
            text = getString(R.string.launcher_description)
            textSize = 15f
            setTextColor(Color.DKGRAY)
            setPadding(0, 0, 0, dp(6))
        }
        root.addView(description)

        selectedStatus = TextView(this).apply {
            textSize = 14f
            setTextColor(Color.DKGRAY)
            isFocusable = true
            setPadding(0, 0, 0, dp(8))
        }
        updateSelectionLabel()
        root.addView(selectedStatus)

        contentRow = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
        }
        root.addView(contentRow, LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT, 0, 1f
        ))

        appScroll = ScrollView(this).apply {
            isFillViewport = true
            isFocusable = true
        }
        val buttons = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
        }
        appScroll.addView(buttons)

        detailPanel = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(dp(20), dp(12), dp(12), 0)
        }
        detailPanel.addView(TextView(this).apply {
            text = getString(R.string.details_title)
            textSize = 20f
            setTextColor(Color.BLACK)
        })
        detailLabel = TextView(this).apply {
            textSize = 16f
            setTextColor(Color.DKGRAY)
            setPadding(0, dp(16), 0, 0)
        }
        detailPanel.addView(detailLabel)
        updateSelectionLabel()
        // Children exist once; resizing only changes their layout properties.
        contentRow.addView(appScroll, LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT, 0, 1f
        ))
        contentRow.addView(detailPanel, LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT
        ))
        detailPanel.visibility = View.GONE

        val query = Intent(Intent.ACTION_MAIN).addCategory(Intent.CATEGORY_LAUNCHER)
        @Suppress("DEPRECATION")
        val installed = packageManager.queryIntentActivities(query, 0)
            .filter { it.activityInfo.packageName != packageName }
            .distinctBy { "${it.activityInfo.packageName}/${it.activityInfo.name}" }
            .sortedBy { it.loadLabel(packageManager).toString().lowercase() }

        for (app in installed) {
            val name = app.loadLabel(packageManager).toString()
            val button = Button(this).apply {
                text = name
                isAllCaps = false
                minHeight = dp(48)
                isFocusable = true // D-pad, keyboard and accessibility traversal.
                contentDescription = getString(R.string.open_app_accessibility, name)
                setOnClickListener {
                    selectedApp = name
                    updateSelectionLabel()
                    try {
                        val launch = Intent(Intent.ACTION_MAIN)
                            .addCategory(Intent.CATEGORY_LAUNCHER)
                            .setClassName(app.activityInfo.packageName, app.activityInfo.name)
                            .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                        startActivity(launch)
                    } catch (_: ActivityNotFoundException) {
                        Toast.makeText(this@MainActivity, R.string.app_unavailable, Toast.LENGTH_SHORT).show()
                    } catch (_: SecurityException) {
                        Toast.makeText(this@MainActivity, R.string.app_not_permitted, Toast.LENGTH_SHORT).show()
                    }
                }
            }
            buttons.addView(button, LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT
            ))
        }

        if (installed.isEmpty()) {
            buttons.addView(TextView(this).apply {
                text = getString(R.string.no_launchable_apps)
                setTextColor(Color.DKGRAY)
            })
        }
        setContentView(root)
    }

    /**
     * Window breakpoints here are experimental MVP defaults, not an official
     * Googlebook certification claim. Expanded supporting pane >= 840dp.
     */
    private fun applyWindowWidthDp(windowWidthDp: Int) {
        val expanded = windowWidthDp >= 840
        if (lastExpanded == expanded) return
        lastExpanded = expanded
        if (expanded) {
            contentRow.orientation = LinearLayout.HORIZONTAL
            appScroll.layoutParams = LinearLayout.LayoutParams(0,
                ViewGroup.LayoutParams.MATCH_PARENT, 0.46f)
            detailPanel.layoutParams = LinearLayout.LayoutParams(0,
                ViewGroup.LayoutParams.MATCH_PARENT, 0.54f)
            detailPanel.visibility = View.VISIBLE
        } else {
            contentRow.orientation = LinearLayout.VERTICAL
            detailPanel.visibility = View.GONE
            appScroll.layoutParams = LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT, 0, 1f
            )
        }
        contentRow.requestLayout()
    }

    private fun updateSelectionLabel() {
        val last = selectedApp
        selectedStatus.text = if (last == null) getString(R.string.no_selection)
                              else getString(R.string.selected_app, last)
        if (::detailLabel.isInitialized) {
            detailLabel.text = if (last == null) getString(R.string.details_hint)
                               else getString(R.string.selected_app, last)
        }
    }

    override fun onSaveInstanceState(outState: Bundle) {
        outState.putString(KEY_LAST_SELECTED, selectedApp)
        super.onSaveInstanceState(outState)
    }

    companion object {
        private const val KEY_LAST_SELECTED = "selected_app_label"
    }
}
