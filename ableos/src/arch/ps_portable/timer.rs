pub fn timer_update() -> ScePspDateTime {
	unsafe {
		let mut tick = 0;
		psp::sys::sceRtcGetCurrentTick(&mut tick);
		// Convert the tick to an instance of `ScePspDateTime`
		let mut date = MaybeUninit::uninit();
		psp::sys::sceRtcSetTick(date.as_mut_ptr(), &tick);
		let date = date.assume_init();
		return date;
	}
}
use core::mem::MaybeUninit;
use psp::sys::ScePspDateTime;
