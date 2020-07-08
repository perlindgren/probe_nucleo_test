# Probe test for SAME54 Xplained-pro

## Setup

- Attach Xplaiened-pro, EDBG connector to host (power only usb-cable), or power the board externally
- Attach the J-link swd programmer to the Xplained pro board and host
- Make sure that no other programmer is attached to the host (or that the Jlink device appears first in the list of programmers, see below).

## Run

``` shell
❯ lsusb
Bus 002 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
Bus 001 Device 002: ID 0cf3:e300 Qualcomm Atheros Communications
Bus 001 Device 003: ID 0c45:6713 Microdia Integrated_Webcam_HD
Bus 001 Device 004: ID 03eb:2111 Atmel Corp. Xplained Pro board debugger and programmer
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
```

Here we see that the SEGGER Device is the only programmer.

> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/probe_connect`
Probe test for SAME54P20A on Xplained-pro with jlink edu
probes : [EDBG CMSIS-DAP (VID: 03eb, PID: 2111, Serial: ATML2748051800005821, DAPLink)]
probe opened
set protocol done
speed_khz 1000
new speed_khz 1000
has dap true
probe attached
core connected
core reset
core halted Ok(false)
core halted Ok(true)
pc 0x00000734
read buff @0x2000_0000
[1007917, 4007086368, 1003994943, 4231805791, 554828289, 813121, 2684364831, 890249281]
increment each word by 1
new content written
read buff @0x2000_0000
[1007918, 4007086369, 1003994944, 4231805792, 554828290, 813122, 2684364832, 890249282]
```

## What is happening?

We exercise basic probing functionality of direct core access using `probe-rs`.

- listing available probes
- connecting to first
- set SWD protocol
- set probe speed
- construct a chip description (from scratch)
- construct a target description (from scratch)
- attach to the target
- reset
- reset and halt
- core register access (reading PC)
- reading and writing memory

## Potential errors

- connection errors (like time-outs will be reported)
- if original memory content is max u32, then the program will panic (due wraparound checking by Rust) unless compiled in --release

## Licence

Free for all.
