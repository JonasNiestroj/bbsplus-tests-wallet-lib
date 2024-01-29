ios:
	@cargo build --release --lib --target aarch64-apple-ios
	@cargo build --release --lib --target aarch64-apple-ios-sim
	@cargo build --release --lib --target x86_64-apple-ios
	@$(RM) -rf libs/bbs-ios.a
	@$(RM) -rf libs/bbs-ios-sim.a
	@cp target/aarch64-apple-ios/release/libbbs.a libs/bbs-ios.a
	@lipo -create -output libs/bbs-ios-sim.a \
			target/aarch64-apple-ios-sim/release/libbbs.a \
			target/x86_64-apple-ios/release/libbbs.a