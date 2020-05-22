#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let input: [u32; 100] = [76663,111378,132647,115688,67473,85562,62955,64052,104961,128687,60344,81158,129984,106462,55967,130004,140810,71523,64891,142922,122783,123918,116246,120842,105578,122950,107512,70051,55347,54348,89301,95258,122323,136781,137756,95658,91017,79626,98414,79296,75226,143850,131334,107028,76591,75492,66400,51904,79262,68956,98957,52481,87955,118871,148734,103699,68681,55118,144120,59403,115012,147742,124218,73580,114949,65346,113104,129059,119068,72339,74984,53095,127452,133786,111439,98153,96312,139641,88907,136831,73574,67871,57641,134505,72116,134503,134387,88598,78687,61020,107234,64801,132668,60204,90001,87833,131148,61488,107938,116072];
    hprintln!("Calculating fuel requirements...").unwrap();
    let mut required_fuel: u32 = 0;

    for value in input.iter() {
        let mut v2 = value.clone();
        loop {
            let fuel = fuel_requirements(v2);
            required_fuel += fuel;
            if fuel > 0 {
                v2 = fuel;
            } else {
                break;
            }
        }
    }

    hprintln!("day1.2: {}", required_fuel).unwrap();

    loop {
        hprintln!("inside loop").unwrap();
    }
}

fn fuel_requirements(mass: u32) -> u32 {
    let f: i32 = (mass as i32 / 3) - 2;

    if f < 0 {
        return 0;
    }

    f as u32
}
