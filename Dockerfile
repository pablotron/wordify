# build stage
#
# notes:
# - disabled upx for now, package is missing (may need to try w/o slim)
# - strip command does not appear to shrink binary
# - need to double-check caching to make sure it is right
FROM docker.io/rust:1-slim AS build
# install upx (disabled, package is missing)
# RUN apt-get update && \
#     apt-get install upx-ucl && \
#     apt-get clean
COPY . /src
WORKDIR /src
RUN --mount=type=cache,target=/root/.rustup ["rustup", "target", "add", "x86_64-unknown-linux-musl"]
RUN --mount=type=cache,target=/root/.cargo ["cargo", "build", "--release", "--target", "x86_64-unknown-linux-musl"]

# pack binary (does not work, see above)
# RUN ["upx", "/src/target/x86_64-unknown-linux-musl/release/wordify"]

# run stage
#
# run with the following command:
#
#   podman --rm -i wordify:latest < examples/hi.json
#
# note: cannot use `-t` or podman fails with an error
FROM scratch AS run
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/wordify /wordify
CMD ["/wordify"]
