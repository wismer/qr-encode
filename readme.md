# QR Encoder

A rust implementation of a QR generator. Uses the [Image](https://github.com/PistonDevelopers/image) cargo package to generate a QR encoded image, and the rust implementation of the [reed-solomon](https://github.com/mersinvald/reed-solomon-rs) algorithm.

## Currently in development, as you can see by these wonderful images!


### When it works....
![Hello?](https://github.com/wismer/qr-encode/blob/master/qr.png)
### When it doesn't...
![Whoops](https://github.com/wismer/qr-encode/blob/master/qr-ex-5.png)
##### Oh man
![Oh man](https://github.com/wismer/qr-encode/blob/master/qr-ex-4.png)
##### What in the world?!
![What in the world?!](https://github.com/wismer/qr-encode/blob/master/qr-ex-4.png)

# Install

run `cargo install qr-encode`

# Use

from the compiled binaries path: `./qr-encode -v <VERSION> -m <MESSAGE>`
