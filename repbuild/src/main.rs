use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Run {
        #[structopt(long)]
        debug: bool,
    },
}

fn build_kernel() -> anyhow::Result<()> {
    let _dir = xshell::pushd("./ableos");

    // Used for the x86-64 variant only
    xshell::cmd!("cargo run --release").run()?;
    // xshell::cmd!("cargo build --release").run()?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Command::from_args();

    match args {
        Command::Run { debug } => {
            build_kernel()?;

            let debug_log: &[&str] = match debug {
                true => &["-D", "debug.log"],
                false => &[],
            };
            #[rustfmt::skip]
            xshell::cmd!("
                qemu-system-x86_64
                    -drive format=raw,file=../../ableos/target/x86_64-ableos/release/bootimage-ableos.bin
                    {debug_log...}
            "
        ).run()?;
        }
    }

    Ok(())
}
