swift-bridge-cli create-package --bridges-dir ./generated --out-dir bbs \
  --ios target/aarch64-apple-ios/debug/libbbs.a \
  --simulator target/universal-ios/debug/libbbs.a \
  --macos target/universal-macos/debug/libbbs.a \
  --name bbs
