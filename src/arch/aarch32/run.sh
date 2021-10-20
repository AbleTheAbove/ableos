sh src/arch/aarch32/build.sh && \
qemu-system-arm -M raspi2 -kernel ableos.elf --serial stdio && \
rm ableos.elf
