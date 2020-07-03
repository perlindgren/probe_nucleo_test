use probe_rs::{Error, MemoryInterface, Probe};

fn probe_test() -> Result<(), Error> {
    // Get a list of all available debug probes.
    let probes = Probe::list_all();
    println!("probes : {:?}", probes);

    // Use the first probe found.
    let probe = probes[0].open()?;

    println!("probe opened");

    // Attach to a chip.
    let mut session = probe.attach("ATSAME54P20A")?;

    println!("probe attached");

    // Select a core.
    let mut core = session.core(0).unwrap();

    core.reset_and_halt()?;

    println!("core halted {:?}", core.core_halted());
    let reg = core.registers();

    // individual registers are accessed by functions
    let pc = reg.program_counter();
    println!("pc {:#010X}", core.read_core_reg(pc)?);

    // Read a block of 8 32 bit words.
    let mut buff = [1234u32; 8];
    core.read_32(0x2000_0000, &mut buff).unwrap();

    println!("read buff @0x2000_0000\n{:?}", buff);

    println!("increment each word by 1");

    // increment the content by 1;
    for i in buff.iter_mut() {
        *i += 1;
    }

    core.write_32(0x2000_0000, &buff)?;
    println!("new content written");

    core.read_32(0x2000_0000, &mut buff)?;

    println!("read buff @0x2000_0000\n{:?}", buff);

    Ok(())
}

fn main() {
    println!("Probe test for nucleo F401RE");
    probe_test().unwrap();
}
