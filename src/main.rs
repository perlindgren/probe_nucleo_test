use probe_rs::{
    config::{Chip, MemoryRegion, RamRegion},
    CoreType, Error, MemoryInterface, Probe, Target, WireProtocol,
};
use std::borrow::Cow;
use std::convert::From;
use std::ops::Range;

fn probe_test() -> Result<(), Error> {
    // Get a list of all available debug probes.
    let probes = Probe::list_all();
    println!("probes : {:?}", probes);

    // Use the first probe found.
    let mut probe = probes[0].open()?;

    println!("probe opened");

    // Set transport.
    probe.select_protocol(WireProtocol::Swd).unwrap();

    println!("set protocol done");

    println!("speed_khz {}", probe.speed_khz());

    // probe.set_speed(1000)?;

    // println!("new speed_khz {}", probe.speed_khz());

    println!("has dap {}", probe.has_dap_interface());

    let chip = Chip {
        name: Cow::from("MyChip"), // just some name
        part: None,
        // the Cow seems overkill
        memory_map: Cow::Borrowed(&[MemoryRegion::Ram(RamRegion {
            range: Range {
                start: 0x2000_0000,
                end: 0x2000_1000,
            },
            is_boot_memory: false,
        })]),
        flash_algorithms: Cow::from(Vec::new()),
    };

    let target = Target::new(
        &chip,
        Vec::new(), // flash_algorithms: Vec<RawFlashAlgorithm>,
        CoreType::M4,
    );

    let mut session = probe.attach(target).unwrap();
    println!("probe attached");

    // Select a core.
    let mut core = session.core(0)?;
    println!("core connected");

    core.reset()?;
    println!("core reset");

    // core.reset_and_halt(std::time::Duration::from_millis(10))?;

    // println!("core halted {:?}", core.core_halted());
    let reg = core.registers();

    // individual registers are accessed by functions
    let pc = reg.program_counter();
    println!("pc {:#010X}", core.read_core_reg(pc)?);

    // // Read a block of 8 32 bit words.
    // let mut buff = [1234u32; 8];
    // core.read_32(0x2000_0000, &mut buff).unwrap();

    // println!("read buff @0x2000_0000\n{:?}", buff);

    // println!("increment each word by 1");

    // // increment the content by 1;
    // for i in buff.iter_mut() {
    //     *i += 1;
    // }

    // core.write_32(0x2000_0000, &buff)?;
    // println!("new content written");

    // core.read_32(0x2000_0000, &mut buff)?;

    // println!("read buff @0x2000_0000\n{:?}", buff);

    Ok(())
}

fn main() {
    println!("Probe test for SAME54P20A on Xplained-pro with jlink edu");
    probe_test().unwrap();
}
