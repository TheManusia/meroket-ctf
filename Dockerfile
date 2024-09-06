FROM docker.io/rust:1-slim-bookworm AS build

ARG pkg=meroket

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

################################################################################

FROM docker.io/debian:bookworm-slim

WORKDIR /app

COPY --from=build /build/main ./

COPY ./file_gambar ./file_gambar
COPY ./5cdf9be3326a66461fbfc32482bd3cceec83e01c02cb2a5f4e2554151e8ed64ea233f7fa4e74babd1d39b874f4b353adc3f8aa9ac2e1c4d393be7dddfd756a90 ./5cdf9be3326a66461fbfc32482bd3cceec83e01c02cb2a5f4e2554151e8ed64ea233f7fa4e74babd1d39b874f4b353adc3f8aa9ac2e1c4d393be7dddfd756a90

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8001

CMD ./main