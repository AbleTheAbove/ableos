use clap::Parser;

#[derive(clap::Parser, Debug)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!("\n"))]
/// Hello Remember this is a feature
enum Command {
    Run {
        #[clap(long, short)]
        debug: bool,

        #[clap(long, short, arg_enum)]
        machine: Option<MachineType>,
    },
}
#[derive(clap::ArgEnum, Debug, Clone)]
enum MachineType {
    X86,
    /// hi
    RISCV,
    ARM,
}

fn main() -> anyhow::Result<()> {
    let args = Command::parse();

    match args {
        Command::Run { debug, machine } => {
            let _dir = xshell::pushd("./ableos");

            let _debug_log: &[&str] = match debug {
                true => &["-D", "debug.log"],
                false => &[],
            };
            match machine.unwrap_or(MachineType::X86) {
                MachineType::X86 => {
                    // Used for the x86-64 variant only
                    xshell::cmd!("cargo run --release").run()?;
                }
                MachineType::ARM => {
                    xshell::cmd!("cargo build --release --target=json_targets/aarch64-ableos.json")
                        .run()?;
                    #[rustfmt::skip]
                    xshell::cmd!(
                        "qemu-system-aarch64
                        -machine virt
                        -m 1024M
                        -cpu cortex-a53
                        -kernel target/aarch64-ableos/release/ableos
                        -device virtio-keyboard
                        "
                    )
                    .run()?;
                }
                MachineType::RISCV => {
                    xshell::cmd!("cargo build --release --target=riscv64gc-unknown-none-elf")
                        .run()?;
                    #[rustfmt::skip]
                    xshell::cmd!(
                        "
                    qemu-system-riscv64
      -machine virt
      -cpu rv64
      -smp 8
      -m 128M
      -bios opensbi-riscv64-generic-fw_jump.bin
      -kernel blog_os/target/riscv64gc-unknown-none-elf/release/blog_os

                        "
                    )
                    .run()?;
                }
            }
            /*
                #[rustfmt::skip]
                xshell::cmd!("
                    qemu-system-x86_64
                        -drive format=raw,file=../../ableos/target/x86_64-ableos/release/bootimage-ableos.bin
                        {debug_log...}
                "
            ).run()?;
            */
        }
    }

    Ok(())
}
