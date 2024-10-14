//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前的 Unix 时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 在 tests7 中，设置环境变量 `TEST_FOO`，其值为当前的 Unix 时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 在 tests8 中，启用 "pass" 特征以使测试用例能够提前返回
    println!("cargo:rustc-cfg=feature=\"pass\"");
}