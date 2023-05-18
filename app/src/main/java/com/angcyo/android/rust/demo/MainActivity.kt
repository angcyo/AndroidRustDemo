package com.angcyo.android.rust.demo

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.main_activity)

        findViewById<TextView>(R.id.text_view)?.text =
            Rust.stringFromRust("${System.currentTimeMillis()}")
    }
}
