Imgsort
=======

A Rust thingy that sorts the pixels in an image.

```bash
git clone https://github.com/alvare/imgsort
cd imgsort
cargo install
imgsort mypic.jpg out.png
```

It will also save the output image in any format
supported by [image](https://docs.rs/image/0.15.0/image/).

## Modes
For now the pixels can be sorted in 7 dimensions:

* the 4 channels in RGBA
* the 3 channels in HSL

For example

```bash
imgsort mypic.jpg lol.jpg --mode hue
```

will sort the pixels according to their hue values.

## Examples

Original:

![Original](/img/pepsi.png?raw=true "Original")

Red sorted:

![Red](/img/pepsi_sorted_red.png?raw=true "Red")

Hue sorted:

![Hue](/img/pepsi_sorted_hue.png?raw=true "Hue")

## Why
I'm just trying out the language 😁.
