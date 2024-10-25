package com.tauri_app.app

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.google.android.material.bottomnavigation.BottomNavigationView

class MainActivity : AppCompatActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(R.layout.activity_main)
    val bottomNavigationView = findViewById<BottomNavigationView>(R.id.btm_nav)
    val homeFragment = HomeFragment()
    val chatFragment = ChatFragment()
    val tauriFragment = TauriFragment()
    bottomNavigationView.setOnItemSelectedListener { item ->
      when (item.itemId) {
        R.id.menu_home -> {
          supportFragmentManager.beginTransaction().replace(R.id.frag_main, homeFragment)
            .commit()
          return@setOnItemSelectedListener true
        }

        R.id.menu_chat -> {
          supportFragmentManager.beginTransaction().replace(R.id.frag_main, chatFragment)
            .commit()
          return@setOnItemSelectedListener true
        }

        R.id.menu_ecommerce -> {
          supportFragmentManager.beginTransaction().replace(R.id.frag_main, tauriFragment)
            .commit()
          return@setOnItemSelectedListener true
        }
      }
      false
    }
    bottomNavigationView.selectedItemId = R.id.menu_home
  }
}