:: TODO: Detect/install rustup
rustup toolchain install stable
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
rustup target add i686-linux-android
"%WINDIR%\System32\bash" --login -c 'rustup toolchain install stable'
"%WINDIR%\System32\bash" --login -c 'rustup toolchain install nightly'
