pub mod drivers;
pub mod init;
#[naked]
#[no_mangle]
unsafe extern "C" fn _boot() -> ! {
    #[rustfmt::skip]
    asm!("
        csrw sie, zero
        csrci sstatus, 2

        .option push
        .option norelax
        lla gp, __global_pointer$
        .option pop

        lla sp, __tmp_stack_top

        lla t0, __bss_start
        lla t1, __bss_end

        1:
            beq t0, t1, 2f
            sd zero, (t0)
            addi t0, t0, 8
            j 1b

        2:
            j {}
    ",
    sym _start, options(noreturn));
}

use crate::serial::SERIAL;

extern "C" fn _start() -> ! {
    SERIAL.lock().out(format_args!("Hi"));
    sloop()
}

pub fn sloop() -> ! {
    loop {
        unsafe { asm!("nop") };
    }
}
