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

COPY --from=build /build/Rocket.tom[l] ./static
COPY --from=build /build/stati[c] ./static

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8001

CMD ./main