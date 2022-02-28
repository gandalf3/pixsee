# Features

## Basics

### Transformations

* Zoom
* Pan
* Rotate (90° offsets, as well as freely/line level)
* Flip

### Keyboard-optimized navigation

Vimmy keys? Blendery keys? modal??
Maybe different presets for different conventions, for ppl more familiar with other varius software?

### Comparison tools

Ideas for back to back image comparison

* Preserve zoom level/pan offset, so same size images can be swapped between without shifting
* Somehow allow re-ordering of images to allow quick toggling through several images to compare them all

### Inspection tools

* Nice histogram
* EXIF data

### Miscellaneous

* Open images from HTTP
* Themeable (think zathura, rofi, etc.)
* Full keyboard support (think feh, zathura)

## All the Formats

* everything supported by freeimage
* heic
* avif
* apng
* we'll have to see about video formats, seems kind of out of scope but if we're doing animated formats anyway..
* ¡ M I D I !

### HDR Support

* Load .hdr and .exr images
* Dynamic range scaling controls, log transform
* Panoramic mapping/viewing?

### Very Large image support

* Tiled loading?
* Start with tiff, possibly expand optimizations to other formats if possible

### Image sequence support?

Maybe, djv_view already does a good job of this for most things, though HDR image sequence viewing would be welcome.
