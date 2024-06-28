# MOSS

Minimal kernal that plays Tetris at boot. All built with Rust

## Installation
```shell
git clone https://github.com/sajjad-source/TetrisKernel.git
cd TetrisKernel
```

### Build Image
```shell
cargo bootimage --release
```

### Build Image and Run in QEMU
```shell
cargo run --release
```
or

### Run on Bare Metal
```shell
dd if=target/x86_64-moss/release/bootimage-moss.bin of=/dev/sdX && sync
```
Replace `sdX` with bootable drive

**NOTE**: All data on `sdX` will be destroyed. Also, *only Legacy BIOS is supported*, so make sure to set that option on the device's BIOS.

### How to Play
The objective of Tetris is to move and rotate falling pieces called tetrominoes in order to create complete horizontal lines. When a line is complete, it will be cleared, and the lines above it will drop down. The game becomes progressively faster as you clear more lines and level up.

### Controls
* `Left Arrow`: Move the active piece left.
* `Right Arrow`: Move the active piece right.
* `Down Arrow`: Move the active piece down (soft drop).
* `Up Arrow`: Rotate the active piece clockwise.
* `Space`: Hard drop the active piece.
* `C`: Hold the active piece.
* `P`: Pause the game.
* `Q`: Quit the game.

