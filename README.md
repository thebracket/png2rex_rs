# rex2png - a utility for converting PNG files into REXPaint files

This is a Rust port of [png2rex](https://github.com/thebracket/png2rex).

## Building png2rex

1. Clone the repository: `git clone https://github.com/thebracket/png2rex_rs.git`
2. Enter the `png2rex_rs` directory: `cd png2rex_rs`
3. Build with `cargo build --release`
4. `png2rex` (`.exe` on Windows) is now in your target/release directory.

## Using png2rex

With the `exe` file, it's as simple as: `png2rex (input png) (output xp)`. For example, `png2rex resources/kitty.png kitty.xp`. If you want to run it with Cargo, you can use `cargo run -- resources/kitty.png kitty.xp`.

## Results

**Original Kitty PNG**:

![Original PNG of a kitty](https://raw.githubusercontent.com/thebracket/png2rex/master/testimage/kitty.png)

**REX Paint result**:

![REXPaint output](https://raw.githubusercontent.com/thebracket/png2rex/master/testimage/Kitty-REX.png)
