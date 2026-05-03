#![no_std]
#![no_main]            
use cortex_m_rt::entry;        // needed for #[entry]
use panic_halt as _;           // panic handler required
use rtt_target::{rprintln, rtt_init_print};

use core::ptr::write_volatile;
const PIN:u32 = 14; //GPIO LED3
const BUTTON:u32 = 4; //GPIO BUTTON3
const GPIO_P1_BASE: u32 = 0x500D8200;
const GPIO_P0_BASE: u32 = 0x5010A000;

// Register Offsets
const PIN_CNF_P1:u32 = GPIO_P1_BASE + 0x80 + (PIN*0x04);
const PIN_CNF_P0: u32 = GPIO_P0_BASE + 0x80 + (BUTTON*0x04);
const OUTSET_P1:u32 = GPIO_P1_BASE + 0x04;
const OUTCLR_P1:u32 = GPIO_P1_BASE + 0x08;
const P0_IN: u32 = GPIO_P0_BASE + 0x0C;

fn write_reg(addr: u32, val: u32) {
    unsafe {
        write_volatile(addr as *mut u32, val);

        }
    
}

fn delay() {
    for _ in 0..1000000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let pin = 14;
    let butt = 4;

    write_reg(PIN_CNF_P1, 0x01); // set as output
    write_reg(PIN_CNF_P0, 0x0C); //set as input and pullup

    loop {
        rprintln!("Polling button state...\n");

        let button_state = unsafe { core::ptr::read_volatile(P0_IN as *const u32) } & (0x01 << butt); // read button state
        rprintln!("Button state: {}\n", button_state);
        if button_state == 0 {
            rprintln!("Inside\n");

        write_reg(OUTSET_P1, 1 << pin); // on
        delay();
        write_reg(OUTCLR_P1, 1 << pin); // off
        delay();

    }
}
}