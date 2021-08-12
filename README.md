# OS-Overhead
I was wondering how the overhead of an operating system would affect the execution speed of a program. If skipping the Linux kernel by booting into a program could significantly speed up the program, this could be used in many contexts to improve performance.

I used the Blake2 hashing library for Rust to mock the work of the program. For both the Linux test and the bare metal test, I used the  time stamp counter on 64-bit CPU's to profile the execution time.
Philipp Oppermann's blog for a [Rust operating system](https://os.phil-opp.com/) helped me create the standalone bootable binary.

## Tests
In order to test the program on your system run the following commands.
```
cd os

# Build binary
cargo +stable build --release

# Run with one CPU core only
taskset -c 0 ./target/release/os
```

In order to test the bare metal version of the program, run the following commands, then restart your computer into the bootloader for the program.
```
cd bare

# Build with nightly toolchain and release mode
cargo +nightly build --release

# Flash the binary onto the drive.
# Replace /dev/sdX with the correct path to the drive
sudo dd if=target/x86_64-os/release/bootimage-os.bin of=/dev/sdX
```

## Results
My Ubuntu production machine took 6 billion reference clock cycles.

Rebooting into the binary took 8 billion reference clock cycles.

These tests were run around 5 times each way and the Linux test was consistently and significantly faster. The following are a few factors on the Linux test that could have caused this result.
- The `cpuid` serializing instruction ran faster
- The time stamp counter incremented at a slower rate
- The program used multiple logical threads