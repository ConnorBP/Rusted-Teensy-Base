//our custom watchdog implementation for Cortex-M Microprocessor
// we need to provide these manually to make it happy :)

use core::arch::arm::__nop;

#[repr(C,packed)]
pub struct Watchdog {
    stctrlh: u16,
    stctrll: u16,
    tovalh: u16,
    tovall: u16,
    winh: u16,
    winl: u16,
    refresh: u16,
    unlock: u16,
    tmrouth: u16,
    tmroutl: u16,
    rstcnt: u16,
    presc: u16
}

impl Watchdog {
    //an unsafe function to make a new watchdog
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x40052000 as *mut Watchdog)
    }

    //safe function to disable watchdog following the procedure set forth in the manufacturer’s data sheet.
    pub fn disable(&mut self) {
        unsafe {

            /* All of our memory access are volatile.
            This tells the Rust compiler that the read (or write) has an effect that it can’t see from our program code.
            In this case, that effect is a hardware access. Without marking our memory accesses volatile,
            the Rust compiler would be free to say “You never read from unlock,
            so I will optimize away the unneeded write to it”.
            This would, naturally, cause our code to fail. */

            core::ptr::write_volatile(&mut self.unlock, 0xC520);
            core::ptr::write_volatile(&mut self.unlock, 0xD928);

            //tells the processor to briefly do nothing. (2 cycles)
            //__NOP();//depricated
            //__NOP();
            //new method of doing this:
            //asm!("nop" : : : "memory");
            //asm!("nop" : : : "memory");
            __nop();
            __nop();
            //ead the control register and un-set the “enable” bit.
            let mut ctrl = core::ptr::read_volatile(&self.stctrlh);
            ctrl &= !(0x00000001);
            core::ptr::write_volatile(&mut self.stctrlh, ctrl);

            /* This disable process shows why we must have only one mutable reference to the watchdog.
            If an interrupt were to occur partway through this function and write to the watchdog,
            our attempt to disable it would fail. Knowing that an interrupt cannot change watchdog settings
            gives us confidence that this code will execute as we expect. */
        }
    }
}
