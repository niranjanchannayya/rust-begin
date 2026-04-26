#![no_std]
#![no_main]            
use cortex_m_rt::entry;        // needed for #[entry]
use panic_halt as _;           // panic handler required

use core::ptr::write_volatile;
const PIN:u32 = 10; //GPIO PIN
const GPIO_P1_BASE: u32 = 0x500D8200;

// Register Offsets
const PIN_CNF:u32 = GPIO_P1_BASE + 0x80 + (PIN*0x04);
const OUTSET:u32 = GPIO_P1_BASE + 0x04;
const OUTCLR:u32 = GPIO_P1_BASE + 0x08;

fn write_reg(addr: u32, val: u32) {
    unsafe {
        write_volatile(addr as *mut u32, val);
        }
    
}

fn delay() {
    for _ in 0..10000000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    let out = 0;
    let pin = 10;

    write_reg(PIN_CNF, 0x01); // set as output

    loop {
        write_reg(OUTSET, 1 << pin); // on
        delay();
        write_reg(OUTCLR, 1 << pin); // off
        delay();
    }
}