use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub fn init_idt() {
   IDT.load();
}

lazy_static! {
   static ref IDT: InterruptDescriptorTable = {
      let mut idt = InterruptDescriptorTable::new();
      idt.breakpoint.set_handler_fn(breakpoint_handler);
      idt.page_fault.set_handler_fn(page_fault_handler);
      idt.double_fault.set_handler_fn(double_fault_handler);
      idt
   };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
   println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
   stack_frame: InterruptStackFrame,
   error_code: PageFaultErrorCode,
) {
   println!("EXCEPTION: PAGEFAULT\n{:#?}\nERROR CODE: {:#?}", stack_frame, error_code);
}

extern "x86-interrupt" fn double_fault_handler(
   stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
   panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}