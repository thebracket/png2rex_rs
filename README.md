# png2rex_rs - a utility for converting PNG files into REXPaint files

This is a Rust port of [png2rex](https://github.com/thebracket/png2rex).

## Building png2rex

1. Clone the repository: `git clone https://github.com/thebracket/png2rex_rs.git`
2. Enter the `png2rex_rs` directory: `cd png2rex_rs`
3. Build with `cargo build --release`
4. `png2rex` (`.exe` on Windows) is now in your target/release directory.

## Using png2rex

With the `exe` file, it's as simple as: `png2rex (input png) (output xp)`. For example, `png2rex resources/kitty.png kitty.xp`. If you want to run it with Cargo, you can use `cargo run -- resources/kitty.png kitty.xp`.

### Optional extras

You can provide additional CLI arguments to perform some image operations en route:

* `--flipv` flips the image vertically.
* `--fliph` flips the image horizontally.
* `--resize 64 32` will resize the image to be 64 pixels wide, 32 pixels tall.

These options can be combined and can be in any order. For example:

```
cargo run -- --fliph .\resources\kitty.png --flipv C:\Users\herbe\Downloads\REXPaint-v1.04\images\kitty.xp --resize 64 64
```

Has the same effect as:

```
cargo run -- .\resources\kitty.png C:\Users\herbe\Downloads\REXPaint-v1.04\images\kitty.xp --resize 64 64 --fliph --flipv 
```

## Results

**Original Kitty PNG**:

![Original PNG of a kitty](https://raw.githubusercontent.com/thebracket/png2rex/master/testimage/kitty.png)

**REX Paint result**:

![REXPaint output](https://raw.githubusercontent.com/thebracket/png2rex/master/testimage/Kitty-REX.png)
