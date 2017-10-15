Minimal example reproducing rust-mozjs #346
===========================================

Checkout this repository and try to build it for Android. You will run into [rust-mozjs issue #346][1].

[1]: https://github.com/servo/rust-mozjs/issues/346


Building for Android
--------------------

Building something for Android means running:

    cargo build --target arm-linux-androideabi

For mozjs there are additional environment variables needed. Here's what works on my machine, please adapt accordingly:

    PATH=/home/vmx/src/rust/android/android-24-toolchain/bin/:$PATH ANDROID_NDK=/home/vmx/src/rust/android/android-ndk-r14b NDK_ANDROID_VERSION=9 cargo build --target arm-linux-androideabi
