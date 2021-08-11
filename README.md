# OS-Overhead
I was wondering how the overhead of an operating system would affect the execution speed of a program.
// TODO

## Tests
In order to test the program on your system run the following commands.
```
cd os
cargo run --release
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