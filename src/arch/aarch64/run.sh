cargo build --target=json_targets/aarch64-ableos.json &&\
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -kernel target/aarch64-unknown-none/debug/ableos
