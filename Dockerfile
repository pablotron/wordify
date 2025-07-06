# build stage
#
# notes:
# - disabled upx for now, "upx-ucl" package is missing in 1 and 1-slim
#   base images.
# - strip command does shrink binary
FROM docker.io/rust:1-slim AS build

# install upx (disabled, see notes)
# RUN apt-get update && \
#     apt-get install upx-ucl && \
#     apt-get clean

# copy source, set work dir
COPY . /src
WORKDIR /src

# build steps:
# 1. set toolchain/target
# 2. build static binary
# 3. copy static binary to `/src/wordify`
#
# note: step #3 is needed because COPY below won't interpolate $(arch)
RUN --mount=type=cache,target=/usr/local/rustup \
    --mount=type=cache,target=/usr/local/cargo/registry \
  rustup default stable && \
  rustup target add $(arch)-unknown-linux-musl && \
  cargo build --release --target $(arch)-unknown-linux-musl && \
  mv target/$(arch)-unknown-linux-musl/release/wordify /src/

# pack binary (disabled, see notes above)
# RUN ["upx", "/src/target/$(arch)-unknown-linux-musl/release/wordify"]

# run stage
#
# run with the following command:
#
#   $ podman --rm -i wordify:latest < examples/hi.json
#
# note: cannot run with `-t` or podman fails with an error
FROM scratch AS run
COPY --from=build /src/wordify /wordify
CMD ["/wordify"]
