arm-none-eabi-gcc -mcpu=cortex-a7 -fpic -ffreestanding -c src/arch/aarch32/boot.s -o boot.o
mv src/main.rs src/main.rs.pass
cargo build --release --target json_targets/aarch32-ableos.json
arm-none-eabi-gcc -T src/arch/aarch32/linker.ld -o ableos.elf -ffreestanding -O2 -nostdlib boot.o \
target/aarch32-ableos/release/libableos.rlib
mv src/main.rs.pass src/main.rs
rm boot.o
