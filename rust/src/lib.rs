extern crate jni;

use std::ffi::CString;
use std::os::raw::c_char;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

/**
 * Rust 代码修改之后, 记住要rebuild
 * https://docs.rs/jni/0.21.1/jni/
 */
/*#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_angcyo_android_rust_demo_Rust_stringFromRust(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String =
        env.get_string(&input).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_raw()
}*/

/**
 * 另一种写法
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_angcyo_android_rust_demo_Rust_stringFromRust<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String =
        env.get_string(&input).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output
}