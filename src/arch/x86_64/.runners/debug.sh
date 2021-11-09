cargo bootimage --target json_targets/x86_64-ableos.json && \
qemu-system-x86_64 -S -gdb tcp::9000 -drive format=raw,file=target/x86_64-ableos/debug/bootimage-ableos.bin
