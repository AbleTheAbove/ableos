sh src/arch/arm/build.sh
qemu-system-arm -M raspi2 -kernel ableos.elf --serial stdio
rm ableos.elf
