# Bytekot painter

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)
[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)

An image-painting library for the Bytekot Telegram bot designed to generate images from textual representations of bytecode with highlighting.
It is also planned to be used in javap-viewer.

The concept is straightforward: the library, for now, exposes a single method, `paint` (and `free_image_data`), which accepts one argument, "C-string", `input` and returns struct with data.
The former is the actual JVM bytecode, while the latter is the path where the image will be saved (depending on the operating system, relative and absolute paths depend on a
forward slash or dot with a forward slash).

## Bugs and improvements

- Any bug fixes and improvements are welcome :) https://github.com/bytekodex/bytekot-painter/pulls
- Any bug reports, suggestions are also welcome https://github.com/bytekodex/bytekot-painter/issues

## Docker image

The ready-made image (`linux/amd64`) is available on the official Docker Hub under the name `bytecodex/bytekot-painter`

```shell
docker push bytecodex/bytekot-painter:v1.1.3
```

For multi-stage builds, in a similar manner (it contains only the dynamic library file and header file (`bytekot_painter.dll/so`,`bytekot_painter.h`))

```dockerfile
FROM bytecodex/bytekot-painter:v1.1.3 as bytekot-painter
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
java -jar rust-antlr.jar -Dlanguage=Rust antlr/JBytecodeParser.g4 antlr/JBytecodeLexer.g4 -o ./src/antlr/
```

2. We'll compile the library (by default, a dynamic library `.dll` on Windows and `.so` on Linux is generated)

```shell
cargo build --release
```

3. The compiled library will be in `./target/release` named `bytekot_painter.dll` or on Linux `libbytekot_painter.so`.

### Docker

![](/nothing/docker-meme.jpg)

```shell
docker build -t bytecodex/bytekot-painter:v1.1.3 .
```

## Example of result

Input parameters: Bytecode of `java.util.jar.JarEntry` gathered from `javap` with `-c` argument.

Result:

![](/nothing/snapshot-result.png)

## API

The API of the library is very simple, just two functions, here's the header for clarity.

```c
typedef struct ImageResult {
  const unsigned char *data;
  uintptr_t len;
  int status;
} ImageResult;

struct ImageResult paint(const char *input);

void free_image_data(unsigned char *ptr, uintptr_t len);
```

The `paint` function takes a regular C string with bytecode for processing and always returns an `ImageResult` struct.

`data` is a pointer to the array data, which can subsequently be written to a file; this array of bytes is in `PNG` format.

`len` is the length of the array.

`status` is the status of the operation, with error codes presented below:

```rust
const ERR_SUCCESS: i32 = 0;
const ERR_TOO_LARGE_IMAGE: i32 = -1;
const ERR_RASTER_CREATION_FAILURE: i32 = -2;
const ERR_IMAGE_ENCODING_FAILURE: i32 = -3;
```

_Note, `ERR_TOO_LARGE_IMAGE` returns only if height greater than `8192` or `height * width` > `27852800`._

Accordingly, to avoid creating a memory leak, there is a `free_image_data` function that takes a pointer to the array and its length.

## Used technologies

- [Rust](https://github.com/rust-lang/rust), licensed with [MIT](https://github.com/rust-lang/log/blob/master/LICENSE-MIT)
  and [Apache 2.0](https://github.com/rust-lang/log/blob/master/LICENSE-APACHE)
- [Antlr4](https://github.com/antlr/antlr4), licensed with [BSD 3](https://github.com/antlr/antlr4/blob/dev/LICENSE.txt)
- [Skia](https://github.com/google/skia), licensed with [BSD 3](https://github.com/google/skia/blob/main/LICENSE)
- [Jetbrains AI](https://www.jetbrains.com/ai/), used for commit names