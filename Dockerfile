# build stage
#
# notes:
# - disabled upx for now, "upx-ucl" package is missing in 1 and 1-slim
#   base images.
# - strip command does shrink binary
FROM docker.io/rust:1-slim AS build

# install upx (disabled, see above)
# RUN apt-get update && \
#     apt-get install upx-ucl && \
#     apt-get clean

# copy source, set work dir
COPY . /src
WORKDIR /src

# set toolchain and target, then build static binary
RUN --mount=type=cache,target=/usr/local/rustup \
    --mount=type=cache,target=/usr/local/cargo/registry \
  rustup default stable && \
  rustup target add x86_64-unknown-linux-musl && \
  cargo build --release --target x86_64-unknown-linux-musl

# pack binary (disabled, see notes above)
# RUN ["upx", "/src/target/x86_64-unknown-linux-musl/release/wordify"]

# run stage
#
# run with the following command:
#
#   $ podman --rm -i wordify:latest < examples/hi.json
#
# note: cannot run with `-t` or podman fails with an error
FROM scratch AS run
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/wordify /wordify
CMD ["/wordify"]
