package com.angcyo.android.rust.demo

/**
 * @author <a href="mailto:angcyo@126.com">angcyo</a>
 * @since 2023/05/18
 */
object Rust {

    init {
        System.loadLibrary("rustFirst")
    }

    external fun stringFromRust(input: String): String
}