# Probe test for SAME54 Xplained-pro

## Setup

- Attach Xplaiened-pro, EDBG connector to host (power only usb-cable), or power the board externally
- Attach the J-link swd programmer to the Xplained pro board and host
- Make sure that no other programmer is attached to the host (or that the Jlink device appears first in the list of programmers, see below).

## Run

``` shell
> lsusb
Bus 002 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
Bus 001 Device 002: ID 0cf3:e300 Qualcomm Atheros Communications
Bus 001 Device 015: ID 1366:0101 SEGGER J-Link PLUS
Bus 001 Device 003: ID 0c45:6713 Microdia Integrated_Webcam_HD
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
```

Here we see that the SEGGER Device is the only programmer.

> cargo run
   Compiling probe_connect v0.1.0 (/home/pln/rust/grepit/st/probe_connect)
    Finished dev [unoptimized + debuginfo] target(s) in 1.76s
     Running `target/debug/probe_connect`
Probe test for nucleo F401RE
probes : [STLink V2-1 (VID: 0483, PID: 374b, Serial: 0676FF323535474B43024732, STLink)]
probe opened
probe attached
core halted Ok(true)
pc 0x08000DD2
read buff @0x2000_0000
[65540, 259, 167772163, 50462979, 117835015, 2315, 3, 3]
increment each word by 1
new content written
read buff @0x2000_0000
[65541, 260, 167772164, 50462980, 117835016, 2316, 4, 4]
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
