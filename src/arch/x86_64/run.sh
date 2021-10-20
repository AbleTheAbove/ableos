sh src/arch/x86_64/build.sh

cargo bootimage --release --target json_targets/x86_64-ableos.json && \
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
