use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[no_mangle]
pub extern "system" fn Java_LogProcessor_countErrors(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jint {
    // 1. Get the file path from Java
    let path: String = env.get_string(&input).expect("Missing path").into();

    // 2. Open the file and count errors (Rust's performance layer)
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return -1, // Return error code
    };

    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        if let Ok(l) = line {
            if l.contains("ERROR") {
                count += 1;
            }
        }
    }

    count as jint
}