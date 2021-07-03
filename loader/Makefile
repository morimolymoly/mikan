build:
	cargo +nightly build --release

run: build
	mkdir -p mnt/EFI/BOOT
	cp ./target/x86_64-unknown-uefi/release/loader.efi mnt/EFI/BOOT/BOOTX64.EFI
	cp ../kernel/target/x86_64-unknown-linux-gnu/release/kernel ./mnt
	qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:mnt -monitor stdio