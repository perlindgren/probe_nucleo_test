use probe_rs::{Error, MemoryInterface, Probe};

fn probe_test() -> Result<(), Error> {
    // Get a list of all available debug probes.
    let probes = Probe::list_all();
    println!("probes : {:?}", probes);

    // Use the first probe found.
    let mut probe = probes[0].open().unwrap();

    // Attach to a chip.
    let mut session = probe.attach("STM32F401RETx").unwrap();

    // Select a core.
    let mut core = session.core(0).unwrap();

    core.reset_and_halt(std::time::Duration::from_millis(10))
        .unwrap();

    println!("core halted {:?}", core.core_halted());
    let reg = core.registers();
    // println!("registers {:?}", registers);
    let pc = reg.program_counter();

    println!("pc {:#010X}", core.read_core_reg(pc).unwrap());

    // Read a block of 50 32 bit words.
    let mut buff = [1234u32; 50];
    core.read_32(0x2000_0000, &mut buff).unwrap();

    println!("buff {:?}", &buff[0..7]);

    // // Read a single 32 bit word.
    // let word = core.read_word_32(0x2000_0000)?;

    // Writing is just as simple.
    let buff = [1234u32; 50];
    core.write_32(0x2000_0000, &buff)?;

    // Read a block of 50 32 bit words.
    let mut buff = [1234u32; 50];
    core.read_32(0x2000_0000, &mut buff).unwrap();

    println!("buff {:?}", &buff[0..7]);

    // // of course we can also write 8bit words.
    // let buff = [0u8; 50];
    // core.write_8(0x2000_0000, &buff)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");
    probe_test().unwrap();
}
