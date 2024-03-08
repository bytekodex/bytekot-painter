# Bytekot painter

An image-painting library for the Bytekot Telegram bot designed to generate images from textual representations of bytecode with highlighting. 
It is also planned to be used in javap-viewer.

The concept is straightforward: the library, for now, exposes a single method, `paint`, which accepts two arguments, "C-strings", `input` and `path`.
The former is the actual JVM bytecode, while the latter is the path where the image will be saved (depending on the operating system, relative and absolute paths depend on a forward slash or dot with a forward slash).

After the image is saved, the library returns control with a C-style string (either the path to the image or a text error).

In the future, I plan to change the interface to return an integer code for errors. (PRs are welcome)

## Bugs and improvements

- Any bug fixes and improvements are welcome :) https://github.com/bytekodex/bytekot-painter/pulls
- Any bug reports, suggestions are also welcome https://github.com/bytekodex/bytekot-painter/issues

## Docker image

The ready-made image (`linux/amd64`) is available on the official Docker Hub under the name `bytecodex/bytekot-painter`

```shell
docker push bytecodex/bytekot-painter:v1.0.0
```

For multi-stage builds, in a similar manner (it contains only the static library file)

```dockerfile
FROM bytecodex/bytekot-painter:v1.0.0 as bytekot-painter
```

## Build

### Bare Metal

#### Requirements:

- JDK (or JRE) 11 version or higher.
- Rust 1.76.0 or higher.

#### Steps

1. We'll generate the antlr lexer and parser

```shell
java -jar rust-antlr.jar -Dlanguage=Rust antlr/JBytecodeParser.g4 antlr/JBytecodeLexer.g4 -o /src/antlr/
```

2. We'll compile the library (by default, a static library `.lib` on Windows and `.a` on Linux is generated)

```shell
cargo build --release
```

3. The compiled library will be in `./target/release` named `bytekot_painter.lib` or on Linux `libbytekot_painter.a`.

### Docker

![](/nothing/docker-meme.jpg)

```shell
docker build -t bytecodex/bytekot-painter:v1.0.0 .
```

## Used technologies

- [Rust](https://github.com/rust-lang/rust), licensed with [MIT](https://github.com/rust-lang/log/blob/master/LICENSE-MIT) and [Apache 2.0](https://github.com/rust-lang/log/blob/master/LICENSE-APACHE)
- [Antlr4](https://github.com/antlr/antlr4), licensed with [BSD 3](https://github.com/antlr/antlr4/blob/dev/LICENSE.txt)
- [Skia](https://github.com/google/skia), licensed with [BSD 3](https://github.com/google/skia/blob/main/LICENSE)
- [Jetbrains AI](https://www.jetbrains.com/ai/), used for commit names