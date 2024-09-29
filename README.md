<img src="readme/thermal.png" width="124" height="124" style="float:right; margin-left: 30px;">

> [!WARNING]
> This project is not yet ready for production use. Contributions and feedback are welcome.

# Receipt Renderer in Rust (ESC/POS)
Thermal is a toolkit for parsing and rendering ESC/POS commands, capable of producing JPEG and HTML outputs.

[Read the docs](https://github.com/zachzurn/thermal/wiki)

## Goals:
* Cover the whole esc/pos spec besides deprecated commands 
* Provide a simple rendering pipeline that makes it easy to render in various formats ✅
* Render to markdown
* Render to an image ✅
* Render to HTML with SVG barcodes and QR Codes ✅
* Allow for the creation of virtual USB and Ethernet printer emulators


## Fonts included in this repo do not fall under this repos licence

See thermal_render/resources/fonts/OFL.txt for the license. Fonts were obtained from JetBrains Mono repository on Github:
https://github.com/JetBrains/JetBrainsMono

## Inspiration/References:
https://github.com/receipt-print-hq/escpos-tools
https://github.com/local-group/rust-escposify
https://github.com/buntine/barcoders
https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=72
