use crate::{arch::gdt, print, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

use crate::arch::sloop;
use x86_64::structures::idt::PageFaultErrorCode;
/// Interrupt offsets.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}
impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
pub fn init_idt() {
    IDT.load();
}
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
      let mut idt = InterruptDescriptorTable::new();
      idt.breakpoint.set_handler_fn(breakpoint_handler);
      unsafe {
          idt.double_fault.set_handler_fn(double_fault_handler)
              .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
      }
      idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
      idt[InterruptIndex::Keyboard.as_usize()] .set_handler_fn(keyboard_interrupt_handler);
      idt
    };
}
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
// new
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    crate::kmain::tick();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use crate::keyboard::{
        CustomLayout, CustomScanCodeSet, DecodedKey, DecodedKeyKind, HandleControl, KeyCode,
        Keyboard,
    };
    use spin::Mutex;
    use x86_64::instructions::port::Port;
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<CustomLayout, CustomScanCodeSet>> =
            Mutex::new(Keyboard::new(
                CustomLayout::new_us104key(),
                CustomScanCodeSet::default(),
                HandleControl::Ignore
            ));
    }
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey {
                    kind: DecodedKeyKind::Unicode,
                    value: character,
                } => {
                    print!("{}", char::try_from(character).unwrap());
                    // serial_print!("{}", character);
                    crate::kmain::key_entropy(character.try_into().unwrap());
                }
                DecodedKey {
                    kind: DecodedKeyKind::RawKey,
                    value: key,
                } => print!("{:?}", KeyCode::from(key)),
            }
        }
    }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    println!["Exception: Page Fault"];
    println!["Address: {:?}", Cr2::read()];
    println!["Error Code: {:?}", error_code];
    println!["{:#?}", stack_frame];
    sloop();
}