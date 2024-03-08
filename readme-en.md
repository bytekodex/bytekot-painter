# Bytekot painter

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)
[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)

An image-painting library for the Bytekot Telegram bot designed to generate images from textual representations of bytecode with highlighting.
It is also planned to be used in javap-viewer.

The concept is straightforward: the library, for now, exposes a single method, `paint` (and `free_paint`), which accepts two arguments, "C-strings", `input` and `path`.
The former is the actual JVM bytecode, while the latter is the path where the image will be saved (depending on the operating system, relative and absolute paths depend on a
forward slash or dot with a forward slash).

After the image is saved, the library returns control with a C-style string (either the path to the image or a text error).

In the future, I plan to change the interface to return an integer code for errors. (PRs are welcome)

## Bugs and improvements

- Any bug fixes and improvements are welcome :) https://github.com/bytekodex/bytekot-painter/pulls
- Any bug reports, suggestions are also welcome https://github.com/bytekodex/bytekot-painter/issues

## Docker image

The ready-made image (`linux/amd64`) is available on the official Docker Hub under the name `bytecodex/bytekot-painter`

```shell
docker push bytecodex/bytekot-painter:v1.0.1
```

For multi-stage builds, in a similar manner (it contains only the static library file)

```dockerfile
FROM bytecodex/bytekot-painter:v1.0.1 as bytekot-painter
```

## Build

### Bare Metal

#### Requirements:

- JDK (or JRE) 11 version or higher.
- Rust 1.76.0 or higher.

#### Steps

0. Fetch git submodules

```shell
git submodule init && git submodule update
```

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
docker build -t bytecodex/bytekot-painter:v1.0.1 .
```

## Example of result

Input parameters: Bytecode of `java.util.jar.JarEntry` gathered from `javap` with `-c` argument.

Result:

![](/nothing/snapshot-result.png)

## Errors which can be returned

| Key                      | Description                                                                   |
|--------------------------|-------------------------------------------------------------------------------|
| `too-large-image`        | Image too large (height greater than `8192` or `height * width` > `27852800`) |
| `image-encoding-failure` | Can't create rasterizer                                                       |
| `file-creation-failure`  | Can't create file                                                             |
| `file-writing-failure`   | Can't write to file                                                           |

## Used technologies

- [Rust](https://github.com/rust-lang/rust), licensed with [MIT](https://github.com/rust-lang/log/blob/master/LICENSE-MIT)
  and [Apache 2.0](https://github.com/rust-lang/log/blob/master/LICENSE-APACHE)
- [Antlr4](https://github.com/antlr/antlr4), licensed with [BSD 3](https://github.com/antlr/antlr4/blob/dev/LICENSE.txt)
- [Skia](https://github.com/google/skia), licensed with [BSD 3](https://github.com/google/skia/blob/main/LICENSE)
- [Jetbrains AI](https://www.jetbrains.com/ai/), used for commit names