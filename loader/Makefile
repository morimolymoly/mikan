build:
	cargo +nightly build -Zbuild-std --target x86_64-unknown-uefi --release

run: build
	mkdir -p mnt/EFI/BOOT
	cp ./target/x86_64-unknown-uefi/release/loader.efi mnt/EFI/BOOT/BOOTX64.EFI
	qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:mnt