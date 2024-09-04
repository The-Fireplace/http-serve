# 1. Build rust
FROM lukemathwalker/cargo-chef:0.1.67-rust-1-slim-bookworm AS chef

RUN update-ca-certificates

# Create appuser
ENV USER=hsdocker
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /httpserve

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /httpserve/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin httpserve

# 2. Package in a small production image
FROM gcr.io/distroless/cc-debian12

# Import user from builder.
COPY --from=chef /etc/passwd /etc/passwd
COPY --from=chef /etc/group /etc/group

WORKDIR /web

# copy the build artifact from the build stage
COPY --from=builder /httpserve/target/release/httpserve ./httpserve

EXPOSE 80

# Use an unprivileged user.
USER hsdocker:hsdocker

# Run the binary
CMD ["./httpserve"]