//based on https://branan.github.io/teensy/2017/01/12/bootup.html
//enables the use of intrinsics, is the reason we need nightly Rust
#![feature(stdsimd)]
//#![feature(asm,panic_info_message)]//if using inline asm for nop instead of https://doc.rust-lang.org/nightly/core/arch/arm/fn.__nop.html
//The Rust standard library relies on a full operating system, and can’t typically be used for embedded development. Instead, we will have access to libcore, which is the subset of std that is available without an OS.
#![no_std]
//the main wrapper is used for application setup tasks that aren’t necessary in embedded programs.
#![no_main]
//disable the built in types
#![no_builtins]

mod port;
mod sim;
mod watchdog;

//
// Main Entry Point / Reset Vector
//

#[no_mangle]//no mangle attribute is needed to tell Rust that these arrays have special meaning at link time. Without it, the data will not appear in our final executable.
pub extern fn main() {//extern makes it safe to use main as our reset vector (main function) by following the C calling convention.
    
    //set up hardware features

    let (wdog,sim,pin) = unsafe {
        (watchdog::Watchdog::new(),
         sim::Sim::new(),
         port::Port::new(port::PortName::C).pin(5))
    };

    wdog.disable();
    sim.enable_clock(sim::Clock::PortC);

    let mut gpio = pin.make_gpio();

    gpio.output();
    gpio.high();

    //our main program loop (should never return)
    loop{

    }
}

/* The Rust compiler relies on certain functionality to be defined by the standard library.
Unfortunately for us, we just disabled it. This means that we are responsible for providing these features.
For now, the only language feature we’re responsible for is the panic handler.
This is the function that gets called to display a message when our code panics.
We will eventually want to pass these messages along to the user,
but initially we will ignore them and hang the program. */

#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
    //hang the program on panic
    loop {};
}


/* _stack_top is not really a function. It is a memory address representing the initial stack pointer.
We pretend that it is a function so that our _VECTORS array is easier to write.
Fortunately calling it from our own code is unsafe,
so we can be pretty sure that only the hardware will read these values. */

extern {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern fn(); 2] = [
    _stack_top,
    main,
];

#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLASHCONFIG: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF
];
