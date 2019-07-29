# Gameboy-Image-Filter
![Input Image](images/lenna_input.png)
![Output Image](images/lenna_output_mid.png)

## How to Build
`cargo build --release`

## How to use
`./gb_filter <input-file> <output-file>` \
To change dithering level: \
`./gb_filter --dithering=[none, low, mid, high] <input-file> <output-file>`
