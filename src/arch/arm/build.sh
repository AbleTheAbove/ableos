arm-none-eabi-gcc -mcpu=cortex-a7 -fpic -ffreestanding -c src/arch/arm/boot.s -o boot.o
cargo build --release --target json_targets/arm-ableos.json

arm-none-eabi-gcc -T src/arch/arm/linker.ld -o ableos.elf -ffreestanding -O2 -nostdlib boot.o \
target/arm-ableos/release/libableos.rlib
rm boot.o
