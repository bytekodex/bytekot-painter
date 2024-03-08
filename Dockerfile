# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.76.0
ARG APP_NAME=bytekot-painter
ARG ARTIFACT_NAME=bytekot_painter

FROM openjdk:21 as antlr-build
WORKDIR /lexer
COPY rust-antlr.jar ./
COPY antlr/JBytecodeLexer.g4 antlr/JBytecodeParser.g4 ./
RUN java -jar rust-antlr.jar -Dlanguage=Rust JBytecodeParser.g4 JBytecodeLexer.g4 -o /src/antlr

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION} AS build
ARG APP_NAME
ARG ARTIFACT_NAME
WORKDIR /lib

COPY --from=antlr-build /src/antlr/ ./src/antlr/
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY . .

RUN --mount=type=cache,target=/lib/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release --target-dir ./target && \
cp ./target/release/lib$ARTIFACT_NAME.a /bin/lib$ARTIFACT_NAME.a
EOF

FROM busybox as final
ARG ARTIFACT_NAME
COPY --from=build /bin/lib$ARTIFACT_NAME.a /lib$ARTIFACT_NAME.a

