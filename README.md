# Probe test for ATSAME54 Xlpained pro

## Setup

- Attach ATSAME board to host
- Make sure that no other programmer is attached to the host (or that the EDBG device appears first in the list of programmers, see below).

## Run

``` shell
> cargo run
   Compiling probe_connect v0.1.0 (/home/pln/rust/grepit/st/probe_connect)
    Finished dev [unoptimized + debuginfo] target(s) in 1.92s
     Running `target/debug/probe_connect`
Probe test for nucleo ATSAME54P20A
probes : [EDBG CMSIS-DAP (VID: 03eb, PID: 2111, Serial: ATML2748051800005821, DAPLink)]
probe opened
probe attached
core halted Ok(true)
pc 0x00000734
read buff @0x2000_0000
[1008885, 3973531921, 1003994944, 4114365281, 554827778, 17590338, 536881184, 554696770]
increment each word by 1
new content written
read buff @0x2000_0000
[1008886, 3973531922, 1003994945, 4114365282, 554827779, 17590339, 536881185, 554696771]
```

## What is happening?

We exercise basic probing functionality of direct core access using `probe-rs`.

- listing available probes
- connecting to first
- choosing target (in this case STM32F401RETx)
- reset and halt
- core register access (reading PC)
- reading and writing memory

## Potential errors

- connection errors (like time-outs will be reported)
- if original memory content is max u32, then the program will panic (due wraparound checking by Rust) unless compiled in --release

## Licence

Free for all.
