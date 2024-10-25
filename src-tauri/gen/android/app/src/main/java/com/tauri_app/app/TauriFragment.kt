package com.tauri_app.app

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.webkit.WebSettings
import android.webkit.WebView
import androidx.fragment.app.Fragment
import app.tauri.plugin.PluginManager


class TauriFragment : Fragment() {
  private lateinit var webView: WebView

  override fun onCreateView(
    inflater: LayoutInflater, container: ViewGroup?,
    savedInstanceState: Bundle?
  ): View? {
    // Inflate the layout for this fragment
    val view = inflater.inflate(R.layout.fragment_tauri, container, false)

    webView = view.findViewById(R.id.webview)
    return view
  }

  override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
    super.onViewCreated(view, savedInstanceState)
    val webSettings: WebSettings = webView.settings
    webSettings.javaScriptEnabled = true  // Enable JavaScript for Tauri
    webSettings.domStorageEnabled = true  // Enable DOM storage if needed

    // This step ensures that the Tauri environment is properly initialized for plugins.
    val tauriActivity = MainTauriActivity()

    // Initialize the webview with Tauri's context
    PluginManager(tauriActivity).onWebViewCreated(webView)
    webView.loadUrl("../../../index.html")
//    webView.loadUrl("file:///android_asset/index.html")

    // Load your app's URL or local assets
//      webView.loadUrl("https://aws.amazon.com/codepipeline/")
  }

  override fun onResume() {
    super.onResume()
    webView.onResume()
  }

  override fun onPause() {
    super.onPause()
    webView.onPause()
  }

  override fun onDestroyView() {
    webView.destroy()
    super.onDestroyView()
  }
}