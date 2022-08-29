# PNG to XPR
Packs a PNG image into Xbox Packed Resource (XPR) format for loading on Xbox

A port of "img-to-xpr.py" to Rust. Only supports PNG files in RGBA format.

Also exports the swizzling functionality as a rust library.

## Usage
```
png_to_xpr_rs --input <INPUT> --output <OUTPUT>
```

```
    -h, --help               Print help information
    -i, --input <INPUT>      Input image
    -o, --output <OUTPUT>    Output file
    -V, --version            Print version information
```
