mod cpu;
mod mem;

fn main() {
    // Fixed Constants
    // External Constants (Will be read from external source in future)
    //let ramsize: usize = 1024*64;

    
    let mut processor: cpu::CPU = cpu::CPU::default();
    println!("{:#06x}", processor.AX);
    println!("{:#06x}", processor.AH);
    println!("{:#06x}", processor.AL);
    processor.initialize();
    println!("\nInit\n");
    println!("{:#06x}", processor.AX);
    println!("{:#06x}", processor.AH);
    println!("{:#06x}", processor.AL);
}
